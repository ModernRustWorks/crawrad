---
id: inspector-model-rows-with-columns
kind: doc
target_file: src/inspector_model.rs
line_hint: 320
author: 
created: 
title: "rows_with_columns"
tags: []
signature: "pub fn rows_with_columns (registry : & Registry , doc : Option < & Document > , selection : & [NodeId] , target : & str , tab : Tab , open : & BTreeSet < String > , folded : & BTreeSet < String > , columns : & dyn Fn (& str) -> ColumnAnswer ,) -> Vec < Row >"
---

Returns inspector rows, resolving dataset columns for the Field row.

The rows of the Object Inspector, resolving dataset columns dynamically for the Field row.
When the columns cannot be read the Field row says why.

# Examples

```rust
use crawrad_actions::inspector_model::{rows_with_columns, Tab};
use crawrad_registry::Registry;
use std::collections::BTreeSet;

let reg = Registry::builtin();
let open = BTreeSet::new();
let folded = BTreeSet::new();
let result = rows_with_columns(&reg, None, &[], "", Tab::Properties, &open, &folded, &|_| None);
assert_eq!(result.len(), 1);
```
