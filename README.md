新しい IMAP client [![Build Status](https://travis-ci.org/GildedHonour/atarashii_imap.svg?branch=master)](https://travis-ci.org/GildedHonour/atarashii_imap) [![crates.io](https://img.shields.io/crates/v/atarashii_imap.svg)](https://crates.io/crates/atarashii_imap)
================================================

新しい (atarashii/new) IMAP client in Rust. It supports plain and secure connections.


## In progress

It's under development...


## Usage

Put this in your `Cargo.toml`:

```toml
[dependencies]
atarashii_imap = "0.2.0"
```


### Example
```rust
extern crate atarashii_imap;
extern crate openssl;

use atarashii_imap::{Connection, Response};
use openssl::ssl::{SslContext, SslStream};
use openssl::ssl::SslMethod::Sslv23;

//.......
match Connection::open_secure("imap.gmail.com", SslContext::new(Sslv23).unwrap(), "gmail_login@gmail.com", "password") {
  Ok(mut conn) => {
    match conn.select("inbox".to_string()) {
      Ok(sel_res) => {
        println!("select cmd result: {}", sel_res);
      },
      Err(e) => println!("error")
    }
    
  },
  Err(e) => panic!("Unable open connection")
}

```

## Commands supported
* select
* examine
* create
* delete
* rename
* subscribe
* unsubscribe
* close
* logout
* capability
* fetch
* copy
* list
* examine
* expunge
* check
* noop


## Author
Alex Maslakov | me@gildedhonour.com

## License
Apache 2.0