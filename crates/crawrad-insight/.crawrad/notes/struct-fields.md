---
id: struct-fields
kind: doc
target_file: src/lib.rs
line_hint: 59
author: 
created: 
title: "struct_fields"
tags: []
signature: "pub fn struct_fields (source : & str , name : & str) -> Option < Vec < Field > >"
---

Parses public fields of a named struct from Rust source code.

The fields of `pub struct <name> { … }` in Rust `source`, in declaration
order; `None` when the struct is not there. Reads the generator's own
shape: one `pub field: Type,` per line, `///` docs above it.

# Examples

```rust
use crawrad_insight::struct_fields;

let src = "pub struct Button {\n    pub text: String,\n}";
let fields = struct_fields(src, "Button").unwrap();
assert_eq!(fields[0].name, "text");
```
