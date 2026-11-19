---
id: client-dapclient-stderr
kind: doc
target_file: src/client.rs
line_hint: 64
author: 
created: 
title: "DapClient::stderr"
tags: []
signature: "pub fn stderr (& self) -> String"
---

What the adapter wrote on stderr so far.

# Examples

```rust,no_run
// Spawning a real adapter requires an installed adapter executable.
use std::path::Path;
use crawrad_dap::DapClient;

if let Ok(client) = DapClient::spawn(Path::new("lldb-dap"), &[]) {
    assert!(client.stderr().is_empty());
}
```
