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

extern crate atarashii_imap;
extern crate openssl;

use atarashii_imap::{Connection, Response};
use openssl::ssl::{SslContext, SslStream};
use openssl::ssl::SslMethod::Sslv23;

fn main() {
  match Connection::open_secure("imap.gmail.com", SslContext::new(Sslv23).unwrap(), "gmail_login@gmail.com", "password") {
    Ok(mut conn) => {
      match conn.select("inbox") {
        Ok(sel_res) => println!("select cmd result: {}", sel_res),
        _ => panic!("select cmd error")
      };
      
    
      println!("\r\n");
      match conn.capability() {
        Ok(Response::Ok(data)) => {
          for x in data.iter() {
            println!("capability cmd item: {}", x);
          }
        },
        _ => panic!("capability cmd error")
      };

      println!("\r\n");
      match conn.list("", "%") {
        Ok(Response::Ok(data)) => {
          for x in data.iter() {
            println!("list cmd item: {}", x);
          }
        },
        _ => panic!("list cmd error")
      };
    },

    Err(e) => panic!("Unable to open connection")
  }
}
