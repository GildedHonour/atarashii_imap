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

use std::net::TcpStream;
use std::io::{Read, Write};
use openssl::ssl::{SslContext, SslStream};
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
  tcp_stream: TcpStream
}

impl Connection {
  pub fn open_plain(host: &str) -> () {
    let tcp_conn = TcpStream::connect((host, TcpStreamSecurity::Plain.port()));
    unimplemented!()
  }

  pub fn open_secure(host: &str, sctx: SslContext) -> () {
    let tcp_conn = TcpStream::connect((host, TcpStreamSecurity::SslTls.port()));
    let ssocket = SslStream::connect(&sctx, tcp_conn);
    unimplemented!()
  
  }

  fn send_cmd(&mut self, cmd: &str) {
    self.tcp_stream.write(cmd.as_bytes());
    unimplemented!()
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
