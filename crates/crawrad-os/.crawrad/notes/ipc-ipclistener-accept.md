---
id: ipc-ipclistener-accept
kind: doc
target_file: src/ipc.rs
line_hint: 180
author: 
created: 
title: "IpcListener::accept"
tags: []
signature: "pub fn accept (& self) -> io :: Result < IpcStream >"
---

Accepts an incoming connection from a client.

# Examples

```rust
use crawrad_os::ipc::{IpcListener, IpcStream};
use std::thread;

let listener = IpcListener::bind("accept-ex").unwrap();
let h = thread::spawn(move || listener.accept());
let _client = IpcStream::connect("accept-ex").unwrap();
assert!(h.join().unwrap().is_ok());
```

# Errors

Returns an error if waiting for or establishing the connection fails.

# Panics

Panics if the internal handle mutex is poisoned.
