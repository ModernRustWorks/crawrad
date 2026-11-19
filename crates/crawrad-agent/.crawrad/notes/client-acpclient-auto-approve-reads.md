---
id: client-acpclient-auto-approve-reads
kind: doc
target_file: src/client.rs
line_hint: 1408
author: 
created: 
title: "AcpClient::auto_approve_reads"
tags: []
signature: "pub fn auto_approve_reads (& self) -> bool"
---

Returns whether read requests are automatically approved.

# Examples

```rust,no_run
// Checks auto approval of reads on client.
use crawrad_agent::AcpClient;
use std::path::Path;

let client = AcpClient::spawn(Path::new("agy"), &[], None, false).unwrap();
assert!(!client.auto_approve_reads());
```
