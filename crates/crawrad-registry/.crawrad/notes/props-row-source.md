---
id: props-row-source
kind: doc
target_file: src/props.rs
line_hint: 81
author: 
created: 
title: "ROW_SOURCE"
tags: []
signature: "pub const ROW_SOURCE : & str = \"DbRowSource\""
---

The capability a Grid's data source asks for: a Connection or a Dataset.

# Examples

```rust
use crawrad_registry::props::ROW_SOURCE;

assert_eq!(ROW_SOURCE, "DbRowSource");
```
