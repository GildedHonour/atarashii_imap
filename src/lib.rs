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

extern crate openssl;
extern crate regex;

use regex::Regex;
use std::net::TcpStream;
use std::io::{Read, Write};
use openssl::ssl;
use std::result;
use std::cell::Cell;

mod error;

pub enum ResponseStatus {
  Ok(Option<Vec<String>>),
  No(Option<Vec<String>>),
  Bad(Option<Vec<String>>)
}

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
  Normal,
  EncryptedPassword,
  Ntlm,
  Kerberos,
  GssApi,
  Skey
}

pub struct Mailbox {

}

enum TcpStreamEx {
  Plain(TcpStream),
  
  //todo
  Ssl(ssl::SslStream<TcpStream>),
  Tls(ssl::SslStream<TcpStream>)
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

impl Connection {
  fn tag_prefix() -> &'static str { 
    "TAG" 
  }

  fn get_tcp_stream(&self) -> TcpStream {
    unimplemented!()
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
            match conn.login_cmd(login, password) {
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

  pub fn open_secure(host: &str, sctx: ssl::SslContext, login: &str, password: &str) -> result::Result<Connection, error::Error> {
    Connection::open_secure2(host, sctx, login, password, TcpStreamSecurity::SslTls.port())
  }

  pub fn open_secure2(host: &str, sctx: ssl::SslContext, login: &str, password: &str, port: u16) -> result::Result<Connection, error::Error> {
    match TcpStream::connect((host, port)) {
      Ok(tcp_conn) => {
        let mut stcp_conn = ssl::SslStream::connect(&sctx, tcp_conn).unwrap();
        let byte_buf: &mut [u8] = &mut [0];
        let mut greet_buf = Vec::new();

        //todo refactor
        //2 const
        loop {
          if greet_buf.len() >= NEW_LINE_FULL_CODE_LEN && &greet_buf[greet_buf.len() - NEW_LINE_FULL_CODE_LEN..] == &NEW_LINE_FULL_CODE[..] {
            break;
          }

          match stcp_conn.read(byte_buf) {
            Ok(x) => greet_buf.push(byte_buf[0]),
            Err(e) => println!("aaa") //todo
          };
        }

        let greeting_re = Regex::new(r"^[*] OK").unwrap(); //todo
        let a1 = String::from_utf8(greet_buf).unwrap();
        if !greeting_re.is_match(&a1) {
            //todo 
          println!("Error, the greeting doesn't match the string OK");
          return Err(error::Error::Connect)
        } else {
          println!("greeting response: {}", a1);
        }
        
        let mut conn = Connection::new(TcpStreamEx::Tls(stcp_conn), host, port);
        //then login_cmd
        match conn.login_cmd(login, password) {
          Ok(login_res) => {
            println!("Login OK");
            Ok(conn)
          },
          Err(e) => Err(error::Error::Login)
        }
      },
      
      Err(e) => panic!("{}", "Unable to connect")
    }
  }
      
  fn login_cmd(&mut self, login: &str, password: &str) -> result::Result<ResponseStatus, error::Error> {
    self.exec_cmd(&format!("LOGIN {} {}", login, password))
  }

  //todo
  pub fn select_cmd(&mut self, mailbox_name: String) -> Result<ResponseStatus, error::Error> {
    match self.exec_cmd(&format!("SELECT {}", mailbox_name)) {
      Ok(ResponseStatus::Ok(Some(data))) => {

        /*
        "* FLAGS (\Answered \Flagge \Draft \Deleted \Seen $Junk $NotJunk $NotPhishing $Phishing Junk NonJunk NotJunk)"
        "* OK [PERMANENTFLAGS (\Answered \Flagged \Draft \Deleted \Seen $Junk $NotJunk $NotPhishing $Phishing Junk NonJunk NotJunk \\*)] Flags permitted."
        "* OK [UIDVALIDITY 643039027] UIDs valid."
        "* 3883 EXISTS"
        "* 0 RECENT"
        "* OK [UIDNEXT 10854] Predicted next UID."
        "* OK [HIGHESTMODSEQ 1838882]"
        "TAG_3 OK [READ-WRITE] INBOX selected. (Success)"
        */

        //todo
        let re_flags = Regex::new(r"FLAGS\s\(.+\)").unwrap();
        let re_exists_num = Regex::new(r"(\d+)\sEXISTS").unwrap();
        let re_recent_num = Regex::new(r"(\d+)\sRECENT").unwrap();

        let mut scr = SelectCmdResponse::default();
        for x in data.iter() {
          if re_flags.is_match(&x) {
            //          scr.flags = Some(...);
            //          let caps = cmd_resp_re.captures(&resp).unwrap();
          }

          if re_exists_num.is_match(&x) {
            let cp = re_exists_num.captures(&x).unwrap();
            scr.exists_num = cp.at(1).unwrap().parse::<u32>().unwrap();
          }

          if re_recent_num.is_match(&x) {
            let cp = re_recent_num.captures(&x).unwrap();
            scr.recent_num = cp.at(1).unwrap().parse::<u32>().unwrap();
          }

        }

        unimplemented!()
      },
      _ => unimplemented!(),
/*
      Ok(ResponseStatus::No(data)) => unimplemented!(),
      Ok(ResponseStatus::Bad(data)) => {
        for x in data.iter() {
          println!("select bad resp item: {:?}", x);
        }

        unimplemented!()
      },
      Err(e) => panic!("select cmd error123")

*/
    }

  }

  //todo
  pub fn examine_cmd(&mut self, mailbox_name: String) -> Result<ResponseStatus, error::Error> {
    match self.exec_cmd(&format!("EXAMINE {}", mailbox_name)) {
      Ok(ResponseStatus::Ok(Some(data))) => {
        unimplemented!()
      },
      _ => unimplemented!(),
    }
  }

  fn exec_cmd(&mut self, cmd: &str) -> Result<ResponseStatus, error::Error> {
    let tag = self.generate_tag();

    //todo refactor
    let stcp_conn = match self.tcp_stream_ex {
      TcpStreamEx::Tls(ref mut x) => x,
      _ => panic!("Unable to deconstruct value")
    };
    
    match stcp_conn.write(format!("{} {}\r\n", tag, cmd).as_bytes()) {
      Ok(x) => {
        let byte_buf: &mut [u8] = &mut [0];
        let mut read_buf: Vec<u8> = Vec::new();
        let regex_str = format!(r"{}\s(OK|NO|BAD){{1}}", tag);  //  let regex_str = format!(r"{}\s(OK\b|NO\b|BAD\b)", tag);
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
            Err(e) => println!("aaa") //todo
          }
        }

        //todo refactor
        let resp = String::from_utf8(read_buf.clone()).unwrap();
        let caps = cmd_resp_re.captures(&resp).unwrap();
        let data = Some(resp.split("\r\n").map(|x| x.to_string()).collect());
        println!("[DEBUG] exec cmd res: {}", caps.at(1).unwrap());//todo remove
        Ok(match caps.at(1) {
          Some("OK") => ResponseStatus::Ok(data),
          Some("NO") => ResponseStatus::No(data),
          Some("BAD") => ResponseStatus::Bad(data),
          _ => panic!("Invalid response")
        })
      },
      _ => Err(error::Error::SendCommand)
    }
  }

  fn generate_tag(&self) -> String {
    let v = self.tag_sequence_number.get();
    self.tag_sequence_number.set(v + 1);
    format!("{}_{}", Connection::tag_prefix(), self.tag_sequence_number.get())
  }

  fn get_current_tag(&self) -> String {
    let v = self.tag_sequence_number.get();
    format!("{}_{}", Connection::tag_prefix(), self.tag_sequence_number.get())
  }
  
// pub fn examine_cmd()
  // pub fn create_cmd()
  // pub fn delete_cmd()
  // pub fn rename_cmd()
  // pub fn subscribe_cmd()
  // pub fn unsubscribe_cmd()
  // pub fn list_cmd()
  // pub fn lsub_cmd()
  // pub fn status_cmd()
  // pub fn append_cmd()
  // pub fn expunge_cmd()
  // pub fn search_cmd()
  // pub fn fetch_cmd()
  // pub fn copy_cmd()
  // Pub Fn store_cmd()
  // pub fn uid_cmd()
  // pub fn check_cmd()
  // pub fn close_cmd()
}

pub struct SelectCmdResponse {
  flags: Option<Vec<String>>, //todo
  exists_num: u32,
  recent_num: u32,
  unseen_num: u32,
  permanent_flags: Option<Vec<String>>, //todo
  uid_next: u32,
  uid_validity: u32
}

impl Default for SelectCmdResponse {
  fn default() -> SelectCmdResponse {
    SelectCmdResponse { 
      flags: None,
      exists_num: 0,
      recent_num: 0,
      unseen_num: 0,
      permanent_flags: None,
      uid_next: 0,
      uid_validity: 0
    }
  }
}

#[cfg(test)]
mod tests {
  #[test]
  fn it_works() {
  }
}
