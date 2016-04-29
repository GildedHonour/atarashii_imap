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

use std::net::TcpStream;
use std::io::Write;

mod error;


pub enum TcpStreamSecurity {
  Basic,
  StartTls,
  SslTls
}

impl TcpStreamSecurity {
  fn port(&self) -> u16 {
    match *self {
      TcpStreamSecurity::Basic | TcpStreamSecurity::StartTls => 143,
      TcpStreamSecurity::SslTls => 993
    }
  }
}

pub enum Authentication {
  Normal,
  EncryptedPassword,
  Ntlm,
  KerberosGssApi
}

pub struct Mailbox {

}

pub struct Client {
  host: String, 
  port: u16,
  tcp_stream: TcpStream
}

impl Client {
  pub fn new(host: &str, tss: TcpStreamSecurity) -> Client {
    let mut stream = TcpStream::connect((host, tss.port()));

    unimplemented!()
  }

  // fn is_secure(&self) -> bool {
  //   self.port == SSL_DEFAULT_PORT
  // }

  // pub fn new_ssl()
  // pub fn new_tls()
  // pub fn connect()

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
