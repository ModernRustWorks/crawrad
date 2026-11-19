---
id: menu-row-json
kind: doc
target_file: src/menu.rs
line_hint: 295
author: 
created: 
title: "row_json"
tags: []
signature: "pub fn row_json (r : & Row) -> serde_json :: Value"
---

One row as `view.get_open_menu` reports it.

# Examples

```rust
use crawrad_actions::menu::row_json;
use crawrad_actions::Row;

let row = Row::rule();
let json = row_json(&row);
assert_eq!(json["rule"], true);
```
