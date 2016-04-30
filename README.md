新しい IMAP client
================================================

新しい (atarashii/new) IMAP client in Rust. It supports plain and secure connections.

### In progress
It's under development...


### Example
```rust
match Connection::open_secure("imap.gmail.com", "login", "password") {
  ResultOk(conn) => {
    let items = conn.list_cmd();
    //......
  },

  Err(e) => 
}

```


## Author
Alex Maslakov

## License
Apache 2.0