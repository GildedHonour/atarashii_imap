新しい IMAP client
================================================

新しい (atarashii/new) IMAP client in Rust. It supports plain and secure connections.

### In progress
It's under development...


### Example
```rust
match Connection::open_secure("imap.gmail.com", SslContext::new(Sslv23).unwrap(), "gmail_login@gmail.com", "password") {
    Ok(mut conn) => {
      match conn.select_cmd("INBOX".to_string()) {
        Ok(sel_res) => {
          println!("select cmd result: \r\n inbox {}\r\n recent: {}\r\n uid validity {}\r\n flags {}\r\n",
                   sel_res.exists_num, 
                   sel_res.recent_num, 
                   sel_res.uid_validity,
                   sel_res.flags.join(", "));
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