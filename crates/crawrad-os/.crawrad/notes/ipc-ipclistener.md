---
id: ipc-ipclistener
kind: doc
target_file: src/ipc.rs
line_hint: 114
author: 
created: 
title: "IpcListener"
tags: []
signature: "pub struct IpcListener"
---

An IPC server listener accepting connections from MCP bridges.

An IPC server listener that accepts incoming connections from MCP bridges.

# Examples

```rust
use crawrad_os::ipc::IpcListener;

let listener = IpcListener::bind("listener-ex").unwrap();
assert_eq!(listener.instance_id(), "listener-ex");
```
