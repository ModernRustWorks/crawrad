---
id: inspector-model-row
kind: doc
target_file: src/inspector_model.rs
line_hint: 187
author: 
created: 
title: "Row"
tags: []
signature: "pub enum Row"
---

One row of the Object Inspector.

The category heading is a **row**, and not a grouping outside the list, for
the same reason as in the Toolbox: the `PortalList` numbers everything it
draws and the click action comes back by index
(`portal_list.rs:563-592`).

# Examples

```rust
use crawrad_actions::inspector_model::Row;

let row = Row::Notice("None");
assert!(matches!(row, Row::Notice(_)));
```
