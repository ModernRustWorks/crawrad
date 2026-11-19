---
id: props-props-data-source
kind: doc
target_file: src/props.rs
line_hint: 1846
author: 
created: 
title: "Props::data_source"
tags: []
signature: "pub fn data_source () -> PropDesc"
---

The connection a bound `Grid` reads its rows from (`DataSource`, #442).

Empty is a grid that shows its own cells; naming a `Connection` makes
it show the table in [`Props::table_name`].

# Examples

```rust
use crawrad_registry::Props;

let p = Props::data_source();
assert_eq!(p.key, "data_source");
```
