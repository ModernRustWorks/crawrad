---
id: kind-name
kind: doc
target_file: src/lib.rs
line_hint: 99
author: 
created: 
title: "kind_name"
tags: []
signature: "pub fn kind_name (kind : portadb :: DataType) -> & 'static str"
---

The column's type as the grid's column defaults read it (#936).

# Examples

```rust
assert_eq!(crawrad_dbprobe::kind_name(portadb::DataType::Real), "REAL");
```
