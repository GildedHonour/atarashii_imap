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
use openssl::ssl::{SslContext, SslStream};
use std::result;

mod error;

pub struct OkResult {
  pub data: Option<Vec<String>>
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

pub struct Connection {
  host: String, 
  port: u16,
  tcp_stream: TcpStream,
  tag: u32
}

const CARRET_RETURN_CHAR: i32 = 0x0d;
const LINE_RETURN_CHAR: i32 = 0x0a;
const MIN_SUCCESSFUL_RESPONSE_LEN: i32 = 2;

impl Connection {

  fn new() -> Connection {
    Connection { tag: 1, port: 1, host: "todo" }
  }

  pub fn open_plain(host: &str, login: &str, password: &str) -> Connection {
    match TcpStream::connect((host, TcpStreamSecurity::Plain.port())) {
      Ok(tcp_conn) => {
        let mut buf = Vec::new();
        let conn = Connection::new();
        match tcp_conn.read_to_end(&mut buf) {
          Ok(bytes_read) => {
            //if OK exists then success

            //then login_cmd
            match conn.login_cmd(login, password) {
              ResultOk(login_res) =>
              LoginError(e) =>
            }
          },

          Err(e) => unimplemented!()
          
        }
      },

      Err(e) => unimplemented!() 
    }
  }

  pub fn open_secure(host: &str, sctx: SslContext, login: &str, password: &str) -> () {
    let tcp_conn = TcpStream::connect((host, TcpStreamSecurity::SslTls.port()));
    let ssocket = SslStream::connect(&sctx, tcp_conn);
    unimplemented!()
  
  }

  fn login_cmd(&self, login: &str, password: &str) -> result::Result<OkResult, error::LoginError> {
    self.send_cmd(format!("LOGIN {} {}", "todo", "todo"))
  }

  fn send_cmd(&mut self, cmd: &str) -> Result<???> {
    let full_cmd = format!("{} {}", self.tag, cmd);
    self.tcp_stream.write(full_cmd.as_bytes());
    unimplemented!()
  }

  fn generate_tag(&self) -> String {
    format!("tag{}", (self.tag += 1).to_string())
  }


  // pub fn select_cmd()
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
  // pub fn store_cmd()
  // pub fn uid_cmd()
  // pub fn check_cmd()
  // pub fn close_cmd()



}

#[cfg(test)]
mod tests {
  #[test]
  fn it_works() {
  }
}
