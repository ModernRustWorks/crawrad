---
id: types-debug-name
kind: doc
target_file: src/types.rs
line_hint: 246
author: 
created: 
title: "debug_name"
tags: []
signature: "pub fn debug_name (name : & str) -> & str"
---

The name `{:?}` prints before a value's fields: none for a tuple.

# Examples

```rust
use crawrad_dap_rust::types::debug_name;

assert_eq!(debug_name("my_crate::Foo"), "Foo");
assert_eq!(debug_name("(i32, i32)"), "");
```
