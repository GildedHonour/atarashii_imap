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
        
        while byte_buf[0] != CARRIAGE_RETURN_CODE && byte_buf[0] != NEW_LINE_CODE {
          match stcp_conn.read(byte_buf) {
            Ok(x) => greet_buf.push(byte_buf[0]),
            Err(e) => println!("aaa") //todo
          }
        }

        //* OK Gimap ready for requests from xxx.aaa.bbb.eee l7mb26996601obn
        let greeting_re = Regex::new(r"^[*] OK").unwrap(); //todo
        if !greeting_re.is_match(&String::from_utf8(greet_buf).unwrap()) {
            //todo 
          println!("Error, the greeting doesn't match the string OK");
          return Err(error::Error::Connect)
        }
            
        let mut conn = Connection::new(TcpStreamEx::Tls(stcp_conn), host, port);
        //then login_cmd
        match conn.login_cmd(login, password) {
          Ok(login_res) => {
            let login_re = Regex::new(r"^* ?????").unwrap();
            //todo check if OK, NO or BAD

            Ok(conn)
          },
          Err(e) => Err(error::Error::Login)
        }
      },
      
      Err(e) => panic!("{}", "Unable to connect")
    }
  }
    
  
  fn login_cmd(&mut self, login: &str, password: &str) -> result::Result<ResponseOk, error::Error> {
    match self.exec_cmd(&format!("LOGIN {} {}", login, password)) {
      Ok(resp_data) => {

        // pasrse the response, check if it's succ-l
        // if "tag OK LOGIN completed"
        // ResponseOk
//        Ok(ResponseOk { data: Vec::new() })
        unimplemented!()
      },

      Err(e) => panic!("Error in login command")
    }    
  }

  fn exec_cmd(&mut self, cmd: &str) -> Result<Vec<String>, error::Error> {
    //todo refactor
    let stcp_conn = match self.tcp_stream_ex {
      TcpStreamEx::Tls(ref mut x) => x,
      _ => panic!("Unable to deconstruct value")
    };
    
    //todo
    match write!(stcp_conn, "{} {}", self.tag_sequence_number.get(), cmd) {
      Ok(_) => {
        let byte_buf: &mut [u8] = &mut [0];
        let mut read_buf = Vec::new();
        while byte_buf[0] != CARRIAGE_RETURN_CODE && byte_buf[0] != NEW_LINE_CODE {
          match stcp_conn.read(byte_buf) {
            Ok(x) => read_buf.push(byte_buf[0]),
            Err(e) => println!("aaa") //todo
          }
        }

        Ok(vec![String::from_utf8(read_buf).unwrap()])
      },
      _ => Err(error::Error::SendCommand)
    }
  }

  fn generate_tag(&self) -> String {
    let v = self.tag_sequence_number.get();
    self.tag_sequence_number.set(v + 1);
    format!("{}_{}", Connection::tag_prefix(), self.tag_sequence_number.get())
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
  // Pub Fn store_cmd()
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
