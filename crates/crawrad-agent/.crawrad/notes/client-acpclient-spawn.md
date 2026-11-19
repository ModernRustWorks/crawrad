---
id: client-acpclient-spawn
kind: doc
target_file: src/client.rs
line_hint: 270
author: 
created: 
title: "AcpClient::spawn"
tags: []
signature: "pub fn spawn (binary_path : & Path , args : & [String] , cwd : Option < & Path > , auto_approve_reads : bool ,) -> Result < Self , String >"
---

Launches an agent subprocess with default permission settings.

# Errors

Returns an error if spawning the process or setting up stdio fails.

# Examples

```rust,no_run
// Spawns external agent process.
use crawrad_agent::AcpClient;
use std::path::Path;

let client = AcpClient::spawn(Path::new("agy"), &[], None, false).unwrap();
assert!(client.is_alive());
```
