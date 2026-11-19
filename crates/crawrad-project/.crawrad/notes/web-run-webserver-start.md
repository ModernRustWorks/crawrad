---
id: web-run-webserver-start
kind: doc
target_file: src/web_run.rs
line_hint: 266
author: 
created: 
title: "WebServer::start"
tags: []
signature: "pub fn start (dir : PathBuf) -> std :: io :: Result < Self >"
---

Starts serving the specified directory on an ephemeral loopback port.

# Errors

Returns the error of binding a free port on `127.0.0.1`.

# Examples

```rust
use crawrad_project::web_run::WebServer;

let server = WebServer::start(std::env::temp_dir());
assert!(server.is_ok());
```
