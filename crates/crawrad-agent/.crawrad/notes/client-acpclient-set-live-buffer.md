---
id: client-acpclient-set-live-buffer
kind: doc
target_file: src/client.rs
line_hint: 1401
author: 
created: 
title: "AcpClient::set_live_buffer"
tags: []
signature: "pub fn set_live_buffer (& self , path : impl Into < String > , content : impl Into < String >)"
---

Sets or updates an in-memory buffer content served to `fs/read_text_file`.

# Examples

```rust,no_run
// Sets live buffer content on client.
use crawrad_agent::AcpClient;
use std::path::Path;

let client = AcpClient::spawn(Path::new("agy"), &[], None, false).unwrap();
client.set_live_buffer("src/main.rs", "fn main() {}");
```
