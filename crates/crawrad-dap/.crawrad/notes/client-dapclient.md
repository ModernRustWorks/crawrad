---
id: client-dapclient
kind: doc
target_file: src/client.rs
line_hint: 11
author: 
created: 
title: "DapClient"
tags: []
signature: "pub struct DapClient"
---

A running debug adapter process and its DAP connection.

A running debug adapter (`lldb-dap`, per ADR 0048) and the conversation
with it. The process is killed when this is dropped.

# Examples

```rust
use std::path::Path;
use crawrad_dap::DapClient;

let res = DapClient::spawn(Path::new("nonexistent-adapter"), &[]);
assert!(res.is_err());
```
