新しい IMAP client
================================================

新しい (atarashii/new) IMAP client in Rust. It supports plain and secure connection.

### In progress
It's under development...


### Example
```rust
  let conn = Connection::open("gmail.com", TcpStreamSecurity::SslTls);
  //......
```


## Author
Alex Maslakov

## License
Apache 2.0