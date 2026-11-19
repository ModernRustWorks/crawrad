---
id: adapter-adapter-file-name
kind: doc
target_file: src/adapter.rs
line_hint: 23
author: 
created: 
title: "adapter_file_name"
tags: []
signature: "pub fn adapter_file_name () -> & 'static str"
---

The adapter's file name on this platform.

# Examples

```rust
use crawrad_dap::adapter::adapter_file_name;

let name = adapter_file_name();
assert!(name.starts_with("lldb-dap"));
```
