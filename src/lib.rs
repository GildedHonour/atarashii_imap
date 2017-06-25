/**
 * Copyright (c) 2016 Alex Maslakov, <http://gildedhonour.com>, <http://alexmaslakov.com>
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 * http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 *
 * For questions and comments about this product, please see the project page at:
 *
 * https://github.com/GildedHonour/atarashii_imap
 *
 */

extern crate native_tls;
extern crate regex;

use regex::Regex;
use std::net::TcpStream;
use std::io::{Read, Write};
use std::result;
use std::cell::Cell;
use std::fmt;


//use native_tls::ssl;
//use native_tls::ssl::{SslMethod, SslConnectorBuilder};
use native_tls::{TlsConnector, TlsConnectorBuilder};

mod error;

pub enum TcpStreamSecurity {
  Plain,
  StartTls,
  SslTls
}

impl TcpStreamSecurity {
  fn port(&self) -> u16 {
    match *self {
      TcpStreamSecurity::Plain | TcpStreamSecurity::StartTls => 143,
      TcpStreamSecurity::SslTls => 993
    }
  }
}

pub enum Authentication {
  NormalPassword,
  EncryptedPassword,
  Ntlm,
  Kerberos,
  TlsCertificate,
  GssApi,
  Skey,
  Oauth2
}

//todo
enum TcpStreamEx {
  Plain(TcpStream),
  StartTls(ssl::SslStream<TcpStream>),
  SslTls(ssl::SslStream<TcpStream>)
}

pub enum Response {
  Ok(Vec<String>),
  No(Vec<String>),
  Bad(Vec<String>)
}

pub enum ResponseOptional {
  Referral,
  Alert,
  Badcharset,
  Parse,
  Permanentflags,
  ReadOnly,
  ReadWrite,
  Trycreate,
  Uidnext,
  Uidvalidity,
  Unseen,
  UnknownCte,
  Uidnotsticky,
  Appenduid,
  Copyuid,
  Urlmech,
  Toobig,
  Badurl,
  Highestmodseq,
  Nomodseq,
  Modified,
  Compressionactive,
  Closed,
  Notsaved,
  Badcomparator,
  Annotate,
  Annotations,
  Tempfail,
  Maxconvertmessages,
  Maxconvertparts,
  Noupdate,
  Metadata,
  Notificationoverflow,
  Badevent,
  UndefinedFilter,
  Unavailable,
  Authenticationfailed,
  Authorizationfailed,
  Expired,
  Privacyrequired,
  Contactadmin,
  Noperm,
  Inuse,
  Expungeissued,
  Corruption,
  Serverbug,
  Clientbug,
  Cannot,
  Limit,
  Overquota,
  Alreadyexists,
  Nonexistent
}

pub struct EmailBox {
  pub flags: Vec<String>,
  pub permanent_flags: Vec<String>,
  pub exists_num: u32,
  pub recent_num: u32,
  pub unseen_num: u32,
  pub uid_next: u32,
  pub uid_validity: u32
}

impl Default for EmailBox {
  fn default() -> EmailBox {
    EmailBox {
      flags: vec![],
      permanent_flags: vec![],
      exists_num: 0,
      recent_num: 0,
      unseen_num: 0,
      uid_next: 0,
      uid_validity: 0
    }
  }
}

impl fmt::Display for EmailBox {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "Exists: {}\r\nRecent: {}\r\nUnseen: {}\r\nUid validity: {}\r\nUid next: {}\r\nFlags: {}\r\nPermanent flags: {}",
           self.exists_num,
           self.recent_num,
           self.unseen_num,
           self.uid_validity,
           self.uid_next,
           self.flags.join(", "),
           self.permanent_flags.join(", "))
  }
}

pub struct Connection {
  host: String,
  port: u16,
  tcp_stream_ex: TcpStreamEx,
  tag_sequence_number: Cell<u32>
}

const CARRIAGE_RETURN_CODE: u8 = 0x0D;
const NEW_LINE_CODE: u8 = 0x0A;
const NEW_LINE_FULL_CODE: [u8; 2] = [CARRIAGE_RETURN_CODE, NEW_LINE_CODE];
const NEW_LINE_FULL_CODE_LEN: usize = 2;

impl fmt::Display for Connection {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "Connection: host: {}, port: {}, tag prefix: {}, tag sequence number: {}",
           self.host, self.port, Connection::tag_prefix(), self.tag_sequence_number.get())
  }
}

impl Connection {
  fn tag_prefix() -> &'static str {
    "TAG"
  }

  fn new(tcps_ex: TcpStreamEx, host: &str, port: u16) -> Connection {
    Connection { port: port, host: host.to_string(), tcp_stream_ex: tcps_ex, tag_sequence_number: Cell::new(1) }
  }

  pub fn open_plain(host: &str, login: &str, password: &str) -> result::Result<Connection, error::Error> {
    Connection::open_plain2(host, login, password, TcpStreamSecurity::Plain.port())
  }

  pub fn open_plain2(host: &str, login: &str, password: &str, port: u16) -> result::Result<Connection, error::Error> {
    match TcpStream::connect((host, port)) {
      Ok(mut tcp_conn) => {
        let mut str_buf = String::new();
        match tcp_conn.read_to_string(&mut str_buf) {
          Ok(bytes_read) => {
            //todo if OK exists then success
            //read greating

            let mut conn = Connection::new(TcpStreamEx::Plain(tcp_conn), host, port);
            match conn.login(login, password) {
              Ok(login_res) => unimplemented!(),
              Err(e) => unimplemented!()
            }
          },

          Err(e) => unimplemented!()
        }
      },

      Err(e) => unimplemented!()
    }
  }

  pub fn open_secure(host: &str, login: &str, password: &str) -> result::Result<Connection, error::Error> {
    Connection::open_secure2(host, login, password, TcpStreamSecurity::SslTls.port())
  }

  pub fn open_secure2(host: &str, login: &str, password: &str, port: u16)
                      -> result::Result<Connection, error::Error> {
    match TcpStream::connect((host, port)) {
      Ok(tcp_conn) => {
        let connector = TslConnectorBuilder::new(SslMethod::tls()).unwrap().build();
        let mut stcp_conn = connector.connect(host, tcp_conn).unwrap();
        Connection::verify_greeting(&mut stcp_conn);
        let tcp_strm = TcpStreamEx::SslTls(stcp_conn);
        let mut conn = Connection::new(tcp_strm, host, port);
        match conn.login(login, password) {
          Ok(_) => Ok(conn),
          Err(_) => Err(error::Error::Login)
        }
      },
      Err(e) => panic!("{}", format!("Unable to connect: {}", e))
    }
  }

  fn verify_greeting(stcp_conn: &mut ssl::SslStream<TcpStream>) -> () {
    let byte_buf: &mut [u8] = &mut [0];
    let mut greet_buf = Vec::new();
    loop {
      if greet_buf.len() >= NEW_LINE_FULL_CODE_LEN &&
        &greet_buf[greet_buf.len() - NEW_LINE_FULL_CODE_LEN..] == &NEW_LINE_FULL_CODE[..] {
          break;
        }

      match stcp_conn.read(byte_buf) {
        Ok(_) => greet_buf.push(byte_buf[0]),
        Err(e) => panic!("Unable to read greeting data from a socket: {}", e)
      };
    }

    let greeting_re = Regex::new(r"^[*] OK").unwrap();
    let maybe_greeting = String::from_utf8(greet_buf).unwrap();
    if !greeting_re.is_match(&maybe_greeting) {
      panic!("Greeting doesn't have the correct format");
    }
  }

 fn select_generic(&mut self, emailbox_name: &str, cmd: &str) -> Result<EmailBox, error::Error> {
    match self.exec_cmd(&format!("{} {}", cmd, emailbox_name)) {
      Ok(Response::Ok(data)) => {
        let re_flags = Regex::new(r"FLAGS\s\((.+)\)").unwrap();
        let re_perm_flags = Regex::new(r"\[PERMANENTFLAGS\s\((.+)\)\]").unwrap();
        let re_uid_validity = Regex::new(r"\[UIDVALIDITY\s(\d+)\]").unwrap();
        let re_exists_num = Regex::new(r"(\d+)\sEXISTS").unwrap();
        let re_recent_num = Regex::new(r"(\d+)\sRECENT").unwrap();
        let re_unseen_num = Regex::new(r"\[UNSEEN\s(\d+)\]").unwrap();
        let re_uid_next = Regex::new(r"\[UIDNEXT\s(\d+)\]").unwrap();
        let re_tag_and_res = Regex::new(&format!(r"{}\s(OK|NO|BAD){{1}}", self.get_current_tag())).unwrap();

        let mut scr = EmailBox::default();
        for x in data.iter() {
          if re_flags.is_match(&x) {
            let cp = re_flags.captures(&x).unwrap();
            let flg1 = cp[1].to_string();
            let flg2: Vec<&str> = flg1.split(" ").collect();
            scr.flags = flg2.iter().map(|x| x.to_string()).collect();
          }

          if re_perm_flags.is_match(&x) {
            let cp = re_perm_flags.captures(&x).unwrap();
            let flg1 = cp[1].to_string();
            let flg2: Vec<&str> = flg1.split(" ").collect();
            scr.permanent_flags = flg2.iter().map(|x| x.to_string()).collect();
          }

          if re_exists_num.is_match(&x) {
            let cp = re_exists_num.captures(&x).unwrap();
            scr.exists_num = cp[1].parse::<u32>().unwrap();
          }

          if re_recent_num.is_match(&x) {
            let cp = re_recent_num.captures(&x).unwrap();
            scr.recent_num = cp[1].parse::<u32>().unwrap();
          }

          if re_uid_next.is_match(&x) {
            let cp = re_uid_next.captures(&x).unwrap();
            scr.uid_next = cp[1].parse::<u32>().unwrap();
          }

          if re_uid_validity.is_match(&x) {
            let cp = re_uid_validity.captures(&x).unwrap();
            scr.uid_validity = cp[1].parse::<u32>().unwrap();
          }

          if re_unseen_num.is_match(&x) {
            let cp = re_unseen_num.captures(&x).unwrap();
            scr.unseen_num = cp[1].parse::<u32>().unwrap();
          }
        }

        Ok(scr)
      },

      _ => unimplemented!(),
/*
      Ok(Response::No(data)) => unimplemented!(),
      Ok(Response::Bad(data)) => {
        for x in data.iter() {
          println!("select bad resp item: {:?}", x);
        }

        unimplemented!()
      },
      Err(e) => panic!("select cmd error123")

*/
    }
 }

  pub fn create(&mut self, mailbox_name: &str) -> Result<Response, error::Error> {
    self.exec_cmd(&format!("create {}", mailbox_name))
  }

  pub fn delete(&mut self, mailbox_name: &str) -> Result<Response, error::Error> {
    self.exec_cmd(&format!("delete {}", mailbox_name))
  }

  pub fn rename(&mut self, current_name: &str, new_name: &str) -> Result<Response, error::Error> {
    self.exec_cmd(&format!("rename {} {}", current_name, new_name))
  }

  pub fn subscribe(&mut self, mailbox_name: &str) -> Result<Response, error::Error> {
    self.exec_cmd(&format!("subscribe {}", mailbox_name))
  }

  pub fn unsubscribe(&mut self, mailbox_name: &str) -> Result<Response, error::Error> {
    self.exec_cmd(&format!("unsubscribe {}", mailbox_name))
  }

  pub fn close(&mut self) -> Result<Response, error::Error> {
    self.exec_cmd(&"close")
  }

  pub fn logout(&mut self) -> Result<Response, error::Error> {
    match self.exec_cmd(&"logout") {
      Ok(Response::Ok(data)) => {
        for x in data.iter() {
          if x.contains("BYE") {
            return Ok(Response::Ok(Vec::default()))
          }
        }

        Ok(Response::Bad(vec!["The response of the server doesn't contain 'BYE'".to_string()]))
      },

      _ => Ok(Response::Bad(Vec::default()))
    }
  }

  pub fn capability(&mut self) -> Result<Response, error::Error> {
    self.exec_cmd(&"capability") //todo -- parse response, remove redundant stuff
  }

  pub fn fetch(&mut self, seq_set: &str, message_data_query: &str) -> Result<Response, error::Error> {
    self.exec_cmd(&format!("fetch {} {}", seq_set, message_data_query))
  }

  pub fn copy(&mut self, seq_set: String, mailbox_name: String) -> Result<Response, error::Error> {
    self.exec_cmd (&format!("copy {} {}", seq_set, mailbox_name))
  }

  pub fn list_all(&mut self) -> Result<Response, error::Error> {
    self.list("", "")
  }

  pub fn list_by_search_query(&mut self, search_pattern: &str) -> Result<Response, error::Error> {
    self.list("", search_pattern)
  }

  pub fn list_by_folder_name(&mut self, folder_name: &str) -> Result<Response, error::Error> {
    self.list(folder_name, "")
  }

  pub fn list(&mut self, folder_name: &str, search_pattern: &str) -> Result<Response, error::Error> {
    self.exec_cmd(&format!("list \"{}\" \"{}\"", folder_name, search_pattern))
  }

  pub fn lsub(&mut self, folder_name: &str, search_pattern: &str) -> Result<Response, error::Error> {
    self.exec_cmd(&format!("lsub \"{}\" \"{}\"", folder_name, search_pattern))
  }

  pub fn select(&mut self, mailbox_name: &str) -> Result<EmailBox, error::Error> {
    self.select_generic(mailbox_name, "select")
  }

  pub fn examine(&mut self, mailbox_name: &str) -> Result<EmailBox, error::Error> {
    self.select_generic(mailbox_name, "examine")
  }

  pub fn expunge(&mut self) -> Result<Response, error::Error> {
    self.exec_cmd(&"expunge")
  }

  pub fn check(&mut self) -> Result<Response, error::Error> {
    self.exec_cmd(&"check")
  }

  pub fn noop(&mut self) -> Result<Response, error::Error> {
    self.exec_cmd(&"noop")
  }

  fn exec_cmd(&mut self, cmd: &str) -> Result<Response, error::Error> {
    let tag = self.generate_tag();

    //todo
    let stcp_conn = match self.tcp_stream_ex {
      TcpStreamEx::SslTls(ref mut x) => x,
      _ => panic!("Unable to deconstruct value the tcp stream variable")
    };

    match stcp_conn.write(format!("{} {}\r\n", tag, cmd).as_bytes()) {
      Ok(_) => {
        let byte_buf: &mut [u8] = &mut [0];
        let mut read_buf: Vec<u8> = Vec::new();
        let regex_str = format!(r"{}\s(OK|NO|BAD){{1}}", tag);
        let cmd_resp_re = Regex::new(&regex_str).unwrap();
        loop {
          if read_buf.len() >= NEW_LINE_FULL_CODE.len() && &read_buf[read_buf.len() - NEW_LINE_FULL_CODE.len()..] == &NEW_LINE_FULL_CODE[..] {
            //todo
            let m1 = String::from_utf8(read_buf.clone()).unwrap();
            if cmd_resp_re.is_match(&m1) {
              break;
            }
          }

          match stcp_conn.read(byte_buf) {
            Ok(_) => read_buf.push(byte_buf[0]),
            Err(e) => println!("Error reading bytes from the socket: {}", e) //todo
          }
        }

        let resp = String::from_utf8(read_buf.clone()).unwrap();
        let caps = cmd_resp_re.captures(&resp).unwrap();
        let data = resp.split("\r\n").map(|x| x.to_string()).collect();
        Ok(match &caps[1] {
          "OK" => Response::Ok(data),
          "NO" => Response::No(data),
          "BAD" => Response::Bad(data),
          _ => panic!("Invalid response")
        })
      },
      _ => Err(error::Error::SendCommand)
    }
  }

  fn login(&mut self, login: &str, password: &str) -> result::Result<Response, error::Error> {
    self.exec_cmd(&format!("LOGIN {} {}", login, password))
  }

  fn generate_tag(&self) -> String {
    let v = self.tag_sequence_number.get();
    self.tag_sequence_number.set(v + 1);
    format!("{}_{}", Connection::tag_prefix(), self.tag_sequence_number.get())
  }

  fn get_current_tag(&self) -> String {
    format!("{}_{}", Connection::tag_prefix(), self.tag_sequence_number.get())
  }

  fn start_tls(&mut self) -> Result<Response, error::Error> {
    self.exec_cmd("starttls")
  }

}

#[cfg(test)]
mod tests {
  #[test]
  fn it_works() {
  }
}
