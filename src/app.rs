extern crate atarashii_imap;
extern crate openssl;

use  atarashii_imap::{Connection};
use openssl::ssl::{SslContext, SslStream};
use openssl::ssl::SslMethod::Sslv23;

fn main() {
  let sctx = SslContext::new(Sslv23);
  let conn = Connection::open_secure("imap.gmail.com", SslContext::new(Sslv23).unwrap(), "aaa", "bbb");
  println!("hellllloo123");
}
