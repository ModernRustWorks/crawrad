---
id: adapter-adapter-dir
kind: doc
target_file: src/adapter.rs
line_hint: 17
author: 
created: 
title: "ADAPTER_DIR"
tags: []
signature: "pub const ADAPTER_DIR : & str = \"lldb\""
---

The folder beside the IDE's executable that holds the adapter.

# Examples

```rust
use crawrad_dap::adapter::ADAPTER_DIR;

assert_eq!(ADAPTER_DIR, "lldb");
```
