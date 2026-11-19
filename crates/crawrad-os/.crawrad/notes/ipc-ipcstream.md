---
id: ipc-ipcstream
kind: doc
target_file: src/ipc.rs
line_hint: 29
author: 
created: 
title: "IpcStream"
tags: []
signature: "pub struct IpcStream"
---

An IPC stream connection between the IDE MCP server and a client.

An IPC stream connection between the IDE MCP server and a client (bridge).

# Examples

```rust
use crawrad_os::ipc::{IpcListener, IpcStream};
use std::thread;

let listener = IpcListener::bind("stream-ex").unwrap();
let h = thread::spawn(move || listener.accept().unwrap());
let stream = IpcStream::connect("stream-ex").unwrap();
let _ = h.join().unwrap();
```
