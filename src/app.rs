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

use  atarashii_imap::{Connection};
use openssl::ssl::{SslContext, SslStream};
use openssl::ssl::SslMethod::Sslv23;

fn main() {
  match Connection::open_secure("imap.gmail.com", SslContext::new(Sslv23).unwrap(), "gildedhonour@gmail.com", "pthgwemnqgvqmgxa") {
    Ok(conn) => {
      let select_res = conn.select_cmd("inbox".to_string());
    },
    Err(e) => panic!("Unable open connection")
  }

//  let conn = Connection::open_secure("imap.mail.ru", SslContext::new(Sslv23).unwrap(), "DqLvvvZHMzrsV2sQDzS757XDa8@mail.ru", "pC7BKWSWg9t5zVKWFRGsy8pEhf");
}
