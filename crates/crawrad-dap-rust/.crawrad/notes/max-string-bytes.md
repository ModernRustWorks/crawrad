---
id: max-string-bytes
kind: doc
target_file: src/lib.rs
line_hint: 31
author: 
created: 
title: "MAX_STRING_BYTES"
tags: []
signature: "pub const MAX_STRING_BYTES : usize = 4096"
---

How many bytes of a string are read before `…`.

# Examples

```rust
use crawrad_dap_rust::MAX_STRING_BYTES;

assert_eq!(MAX_STRING_BYTES, 4096);
```
