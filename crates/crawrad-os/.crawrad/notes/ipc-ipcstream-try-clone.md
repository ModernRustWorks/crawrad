---
id: ipc-ipcstream-try-clone
kind: doc
target_file: src/ipc.rs
line_hint: 82
author: 
created: 
title: "IpcStream::try_clone"
tags: []
signature: "pub fn try_clone (& self) -> io :: Result < Self >"
---

Clones the IPC stream for concurrent reading and writing.

# Examples

```rust
use crawrad_os::ipc::{IpcListener, IpcStream};
use std::thread;

let listener = IpcListener::bind("stream-clone").unwrap();
let h = thread::spawn(move || listener.accept().unwrap());
let stream = IpcStream::connect("stream-clone").unwrap();
assert!(stream.try_clone().is_ok());
let _ = h.join().unwrap();
```

# Errors

Returns an error if cloning the underlying file or socket fails.
