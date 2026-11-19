---
id: adapter-adapter-env
kind: doc
target_file: src/adapter.rs
line_hint: 20
author: 
created: 
title: "ADAPTER_ENV"
tags: []
signature: "pub const ADAPTER_ENV : & str = \"CRAWRAD_LLDB_DAP\""
---

The variable that names an adapter explicitly.

# Examples

```rust
use crawrad_dap::adapter::ADAPTER_ENV;

assert_eq!(ADAPTER_ENV, "CRAWRAD_LLDB_DAP");
```
