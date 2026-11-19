---
id: web-run-webserver
kind: doc
target_file: src/web_run.rs
line_hint: 258
author: 
created: 
title: "WebServer"
tags: []
signature: "pub struct WebServer"
---

HTTP server serving WebAssembly assets with COOP and COEP security headers.

# Examples

```rust
use crawrad_project::web_run::WebServer;
use std::path::PathBuf;

let server = WebServer::start(std::env::temp_dir());
assert!(server.is_ok());
```
