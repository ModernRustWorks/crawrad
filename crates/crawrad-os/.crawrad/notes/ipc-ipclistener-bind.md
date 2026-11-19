---
id: ipc-ipclistener-bind
kind: doc
target_file: src/ipc.rs
line_hint: 128
author: 
created: 
title: "IpcListener::bind"
tags: []
signature: "pub fn bind (instance_id : & str) -> io :: Result < Self >"
---

Binds an IPC server listener for the specified instance ID.

# Examples

```rust
use crawrad_os::ipc::IpcListener;

let listener = IpcListener::bind("bind-ex");
assert!(listener.is_ok());
```

# Errors

Returns an error if the named pipe or socket cannot be created.
