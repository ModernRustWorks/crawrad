---
id: types-pointee-name
kind: doc
target_file: src/types.rs
line_hint: 282
author: 
created: 
title: "pointee_name"
tags: []
signature: "pub fn pointee_name (name : & str) -> & str"
---

Extracts the pointee type name from a pointer or reference string.

The pointee's type of a pointer or reference: `probe::Form *` → `probe::Form`,
`&mut probe::Form` → `probe::Form`.

# Examples

```rust
use crawrad_dap_rust::types::pointee_name;

assert_eq!(pointee_name("&mut probe::Form"), "probe::Form");
```
