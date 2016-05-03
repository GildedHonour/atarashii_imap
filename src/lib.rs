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

pub struct ResponseOk {
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

enum TcpStreamEx {
  Plain(TcpStream),
  Ssl(SslStream<TcpStream>),
  Tls(SslStream<TcpStream>)
}

pub struct Connection {
  host: String, 
  port: u16,
  tcp_stream: TcpStreamEx,
  tag_sequence_number: u32
}

impl Connection {
  fn tag_prefix() -> &'static str { 
    "TAG" 
  }

  fn new(tcps: TcpStreamEx, host: &str, port: u16) -> Connection {
    Connection { port: port, host: host.to_string(), tcp_stream: tcps, tag_sequence_number: 1 }
  }

  pub fn open_plain(host: &str, login: &str, password: &str) -> result::Result<Connection, error::ConnectError> {
    Connection::open_plain2(host, login, password, TcpStreamSecurity::Plain.port())
  }
  
  pub fn open_plain2(host: &str, login: &str, password: &str, port: u16) -> result::Result<Connection, error::ConnectError> {
    match TcpStream::connect((host, port)) {
      Ok(tcp_conn) => {
        let mut str_buf = String::new();
        let mut conn = Connection::new(TcpStreamEx::Basic(tcp_conn), host, port);
        match conn.tcp_stream.read_to_string(&mut str_buf) {
          Ok(bytes_read) => {
            //if OK exists then success
            
            //then login_cmd
            match conn.login_cmd(login, password) {
              Ok(login_res) => unimplemented!(),
              Err(error::LoginError) => unimplemented!()
            }
          },

          Err(e) => unimplemented!()
          
        }
      },

      Err(e) => unimplemented!() 
    }
  }

  pub fn open_secure(host: &str, sctx: SslContext, login: &str, password: &str) -> result::Result<Connection, error::ConnectError> {
    Connection::open_secure2(host, sctx, login, password, TcpStreamSecurity::SslTls.port())
  }

  pub fn open_secure2(host: &str, sctx: SslContext, login: &str, password: &str, port: u16) -> result::Result<Connection, error::ConnectError> {
    match TcpStream::connect((host, port)) {
      Ok(tcp_conn) => {
        let stcp_conn = SslStream::connect(&sctx, tcp_conn).unwrap();
        let mut conn = Connection::new(TcpStreamEx::Tls(stcp_conn), host, port);
        let mut str_buf = String::new();
        match conn.tcp_stream.read_to_string(&mut str_buf) {
          Ok(bytes_read) => {
            //if OK exists then success
            
            //then login_cmd
            match conn.login_cmd(login, password) {
              Ok(login_res) => unimplemented!(),
              Err(error::LoginError) => unimplemented!()
            }
          },

          Err(e) => unimplemented!()          
        }      
      },
      
      Err(e) => panic!("{}", "Unable to connect")
    }
  }
    
  fn login_cmd(&mut self, login: &str, password: &str) -> result::Result<ResponseOk, error::LoginError> {
    match self.send_cmd(&format!("LOGIN {} {}", login, password)) {
      Ok(x) => {
        let mut str_buf = String::new();
        let res = self.tcp_stream.read_to_string(&mut str_buf);
        // pasrse the response, check if it's succ-l
        // if "tag OK LOGIN completed"
        // ResponseOk
        unimplemented!()
      },

      Err(e) => panic!("Error in login command: {}", e)
    }    
  }

  fn send_cmd(&mut self, cmd: &str) -> std::io::Result<usize> {
    let full_cmd = format!("{} {}", self.tag_sequence_number, cmd);
    self.tcp_stream.write(full_cmd.as_bytes())
  }

  fn generate_tag(&self) -> String {
    //self.tag_sequence_number += 1; todo
    format!("{}_{}", Connection::tag_prefix(), self.tag_sequence_number)
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
