---
id: ipc-ipcstream-connect
kind: doc
target_file: src/ipc.rs
line_hint: 39
author: 
created: 
title: "IpcStream::connect"
tags: []
signature: "pub fn connect (instance_id : & str) -> io :: Result < Self >"
---

Connects to a running CrawRAD IDE instance by instance ID.

# Examples

```rust
use crawrad_os::ipc::{IpcListener, IpcStream};
use std::thread;

let listener = IpcListener::bind("stream-conn").unwrap();
let h = thread::spawn(move || listener.accept().unwrap());
let stream = IpcStream::connect("stream-conn");
assert!(stream.is_ok());
let _ = h.join().unwrap();
```

# Errors

Returns an error if connecting to the named pipe or socket fails.
