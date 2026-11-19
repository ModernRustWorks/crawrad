---
id: client-acpclient-cancel
kind: doc
target_file: src/client.rs
line_hint: 1370
author: 
created: 
title: "AcpClient::cancel"
tags: []
signature: "pub fn cancel (& self) -> Result < () , String >"
---

Cancels the currently running prompt turn.

Cancels the currently running prompt turn (`session/cancel`), protected by a timeout.

# Errors

Returns an error if locking or writing to stdin fails.

# Examples

```rust,no_run
// Cancels prompt turn on external agent.
use crawrad_agent::AcpClient;
use std::path::Path;

let client = AcpClient::spawn(Path::new("agy"), &[], None, false).unwrap();
assert!(client.cancel().is_ok());
```
