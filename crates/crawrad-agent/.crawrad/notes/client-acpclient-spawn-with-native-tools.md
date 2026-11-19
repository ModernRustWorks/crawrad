---
id: client-acpclient-spawn-with-native-tools
kind: doc
target_file: src/client.rs
line_hint: 280
author: 
created: 
title: "AcpClient::spawn_with_native_tools"
tags: []
signature: "pub fn spawn_with_native_tools (binary_path : & Path , args : & [String] , cwd : Option < & Path > , auto_approve_reads : bool , allow_native_agent_tools : bool ,) -> Result < Self , String >"
---

Launches an agent subprocess with native tools policy.

Launches an agent subprocess and initializes continuous ACP stdio client with native tools policy.

# Errors

Returns an error if spawning the process or setting up stdio fails.

# Examples

```rust,no_run
// Spawns external agent process.
use crawrad_agent::AcpClient;
use std::path::Path;

let client = AcpClient::spawn_with_native_tools(Path::new("agy"), &[], None, false, false).unwrap();
assert!(client.is_alive());
```
