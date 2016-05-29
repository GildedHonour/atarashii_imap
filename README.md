新しい IMAP client
================================================

新しい (atarashii/new) IMAP client in Rust. It supports plain and secure connections.

### In progress
It's under development...


### Example
```rust
match Connection::open_secure("imap.gmail.com", SslContext::new(Sslv23).unwrap(), "gmail_login@gmail.com", "password") {
  Ok(conn) => {
    let select_res = conn.select_cmd("inbox".to_string());
  },
  Err(e) => panic!("Unable open connection")
}

```


## Author
Alex Maslakov | me@gildedhonour.com

## License
Apache 2.0