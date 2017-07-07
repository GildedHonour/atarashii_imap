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
use native_tls::{TlsConnector, TlsConnectorBuilder, TlsStream, SslMethod, SslConnectorBuilder};

mod error;

pub enum SslMode {
    None,
    Explicit,
    Implicit
}

impl SslMode {
    fn port(&self) -> u16 {
        match *self {
            SslMode::None | SslMode::Explicit => 143,
            SslMode::Implicit => 993
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

//todo
impl Connection {
    pub fn open(host: &str, ssl_mode: SslMode, credentials: (&str, &str))
                -> result::Result<Connection, error::Error> {

        let mut conn = match ssl_mode {
            SslMode::None => {
                match TcpStream::connect((host, SslMode::None.port())) {
                    Ok(mut unsec_conn) => {
                        let mut str_buf = String::new();
                        match unsec_conn.read_to_string(&mut str_buf) {
                            Ok(bytes_read) => {
                                //TODO: if OK exists then success
                                //read greating

                                Connection { port: port, host: host.to_string(),
                                             tag_sequence_number: Cell::new(1) };
                            },

                            Err(e) => unimplemented!()
                        }
                    },

                    Err(e) => unimplemented!()
                }
            },

            SslMode::Implicit => {
                let connector = TlsConnector::builder().unwrap().build().unwrap();

                //todo:  match
                let pl_stream = TcpStream::connect(format!("{}:{}", host, SslMode::None.port())).unwrap();

                //todo:  match
                let mut stream = connector.connect(host, pl_stream).unwrap();
                Connection { host: host.to_string(), tag_sequence_number: Cell::new(1) };
            },

            // todo
            SslMode::Explicit => {
                match TcpStream::connect((host, SslMode::Explicit.port())) {
                    Ok(unsec_conn) => {
                        let connector = TlsConnector::builder().unwrap().build().unwrap();
                        let mut sec_conn = connector.connect(host, unsec_conn).unwrap();
                        Connection::verify_greeting(&mut sec_conn);



                        //todo
                        //match
                        let tls_cmd = start_tls(conn);
                        let mut conn = Connection { port: port, host: host.to_string(),
                                                    tag_sequence_number: Cell::new(1) };
                    },
                    Err(e) => panic!("{}", format!("Unable to connect: {}", e))
                }
            }
        }


        //todo
        match conn.login(credentials) {
            Ok(login_res) => Ok(conn),
            Err(e) => Err(error::Error::Login)
        }
    }

    fn tag_prefix() -> &'static str {
        "TAG"
    }

    fn verify_greeting!(stcp_conn: &mut ssl::SslStream<TcpStream>) {
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

    //commands

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

    fn login(&mut self, credentials: (&str, &str)) -> result::Result<Response, error::Error> {
        let (usr_lgn, pass) = credentials;
        self.exec_cmd(&format!("LOGIN {} {}", usr_lgn, pass))
    }

    fn start_tls(&mut self) -> Result<Response, error::Error> {
        self.exec_cmd("starttls")
    }

    //end commands


    fn generate_tag(&self) -> String {
        let v = self.tag_sequence_number.get();
        self.tag_sequence_number.set(v + 1);
        format!("{}_{}", Connection::tag_prefix(), self.tag_sequence_number.get())
    }

    fn get_current_tag(&self) -> String {
        format!("{}_{}", Connection::tag_prefix(), self.tag_sequence_number.get())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
