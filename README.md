新しい IMAP client [![Build Status](https://travis-ci.org/GildedHonour/atarashii_imap.svg?branch=master)](https://travis-ci.org/GildedHonour/atarashii_imap) [![crates.io](https://img.shields.io/crates/v/atarashii_imap.svg)](https://crates.io/crates/atarashii_imap)
================================================

新しい (atarashii/new) IMAP client in Rust. It supports plain and secure connections.


## In progress

It's under development...


## Usage

Put this in your `Cargo.toml`:

```toml
[dependencies]
atarashii_imap = "<current version of atarashii_imap>"
```


### Example
```rust
extern crate atarashii_imap;
extern crate openssl;

use atarashii_imap::{Client, Response, SslMode};
use native_tls::{TlsConnector, TlsConnectorBuilder, TlsStream, SslMethod, SslConnectorBuilder};
//.......

match Client::connect("imap.gmail.com") {
  Ok(mut client) => {
    match conn.authenticate("login123@gmail.com", "password") {
        //todo

        // doing stuff with client
        // ............

        client.disconnect();
      },

      Err(e) => println!("authentication error")
    }
  },

  Err(e) => panic!("connection error")
}

```

## Commands supported
* select(mailbox_name: &str)
* examine(mailbox_name: &str)
* create(mailbox_name: &str)
* delete(mailbox_name: &str)
* rename(current_name: &str, new_name: &str)
* subscribe(mailbox_name: &str)
* unsubscribe(mailbox_name: &str)
* close
* logout
* capability
* fetch
* copy(seq_set: String, mailbox_name: String)
* list(folder_name: &str, search_pattern: &str)
* lsub(folder_name: &str, search_pattern: &str)
* expunge
* check
* noop


## Author
Alex Maslakov | me@gildedhonour.com

## License
Apache 2.0
