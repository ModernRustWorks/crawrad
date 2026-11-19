---
id: process-processstatus
kind: doc
target_file: src/process.rs
line_hint: 37
author: 
created: 
title: "ProcessStatus"
tags: []
signature: "pub enum ProcessStatus"
---

Lifecycle status of an external extension subprocess.

# Examples

```rust
use crawrad_extension_host::ProcessStatus;

let status = ProcessStatus::Unstarted;
assert_eq!(status, ProcessStatus::Unstarted);
```
