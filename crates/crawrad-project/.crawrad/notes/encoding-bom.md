---
id: encoding-bom
kind: doc
target_file: src/encoding.rs
line_hint: 25
author: 
created: 
title: "BOM"
tags: []
signature: "pub const BOM : & [u8] = & [0xEF , 0xBB , 0xBF]"
---

The three bytes of the UTF-8 byte order mark.

# Examples

```rust
use crawrad_project::encoding::BOM;

assert_eq!(BOM, &[0xEF, 0xBB, 0xBF]);
```
