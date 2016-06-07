新しい IMAP client
================================================

新しい (atarashii/new) IMAP client in Rust. It supports plain and secure connections.

### In progress
It's under development...


### Example
```rust
extern crate atarashii_imap;
extern crate openssl;

use atarashii_imap::{Connection};
use openssl::ssl::{SslContext, SslStream};
use openssl::ssl::SslMethod::Sslv23;

//.......

match Connection::open_secure("imap.gmail.com", SslContext::new(Sslv23).unwrap(), "gmail_login@gmail.com", "password") {
  Ok(mut conn) => {
    match conn.select_cmd("INBOX".to_string()) {
      Ok(sel_res) => {
        println!("select cmd result: {}", sel_res);
      },
      Err(e) => println!("error")
    }
    
  },
  Err(e) => panic!("Unable open connection")
}

```


## Author
Alex Maslakov | me@gildedhonour.com

## License
Apache 2.0