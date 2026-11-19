---
id: props-props-table-name
kind: doc
target_file: src/props.rs
line_hint: 1858
author: 
created: 
title: "Props::table_name"
tags: []
signature: "pub fn table_name (default : & str) -> PropDesc"
---

The table name a bound `Grid` displays (`TableName`, #442).

The table a bound `Grid` shows (`TableName`, #442). Its columns come
from the database when the form opens, so the form does not list them.

# Examples

```rust
use crawrad_registry::Props;

let p = Props::table_name("users");
assert_eq!(p.key, "table_name");
```
