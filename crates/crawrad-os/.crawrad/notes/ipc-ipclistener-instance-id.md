---
id: ipc-ipclistener-instance-id
kind: doc
target_file: src/ipc.rs
line_hint: 175
author: 
created: 
title: "IpcListener::instance_id"
tags: []
signature: "pub fn instance_id (& self) -> & str"
---

Returns the instance ID associated with this listener.

# Examples

```rust
use crawrad_os::ipc::IpcListener;

let listener = IpcListener::bind("id-ex").unwrap();
assert_eq!(listener.instance_id(), "id-ex");
```
