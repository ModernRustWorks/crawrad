---
id: types-is-std-internal
kind: doc
target_file: src/types.rs
line_hint: 275
author: 
created: 
title: "is_std_internal"
tags: []
signature: "pub fn is_std_internal (name : & str) -> bool"
---

Returns whether a type belongs to standard library internal modules.

A type that belongs to the standard library or its dependencies, whose
fields are implementation details.

# Examples

```rust
use crawrad_dap_rust::types::is_std_internal;

assert!(is_std_internal("std::sync::Arc<i32>"));
assert!(!is_std_internal("my_app::Model"));
```
