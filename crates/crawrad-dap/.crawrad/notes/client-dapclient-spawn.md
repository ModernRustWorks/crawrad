---
id: client-dapclient-spawn
kind: doc
target_file: src/client.rs
line_hint: 19
author: 
created: 
title: "DapClient::spawn"
tags: []
signature: "pub fn spawn (program : & Path , args : & [& str]) -> Result < Self , DapError >"
---

Starts `program` with `args` and opens the conversation on its stdio.

Through `crawrad_os::process::no_console`: an adapter is a console program,
and without it Windows opens a console window next to the IDE (#267).

# Examples

```rust
use std::path::Path;
use crawrad_dap::DapClient;

let err = DapClient::spawn(Path::new("nonexistent-adapter"), &[]).err().expect("no such adapter");
assert!(err.to_string().contains("nonexistent-adapter"));
```

# Errors

Returns [`DapError::Spawn`] if the process cannot be started or pipes cannot be opened.
