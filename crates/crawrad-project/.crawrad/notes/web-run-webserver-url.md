---
id: web-run-webserver-url
kind: doc
target_file: src/web_run.rs
line_hint: 295
author: 
created: 
title: "WebServer::url"
tags: []
signature: "pub fn url (& self) -> String"
---

Returns the loopback HTTP URL where the server is reachable.

# Examples

```rust
use crawrad_project::web_run::WebServer;

let server = WebServer::start(std::env::temp_dir()).unwrap();
assert!(server.url().starts_with("http://127.0.0.1:"));
```
