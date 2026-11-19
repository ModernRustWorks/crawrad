---
id: props-props-editing
kind: doc
target_file: src/props.rs
line_hint: 1835
author: 
created: 
title: "Props::editing"
tags: []
signature: "pub fn editing (default : bool) -> PropDesc"
---

Whether the user can edit non-fixed cells in a Grid (`Editing`, #442).

# Examples

```rust
use crawrad_registry::Props;

let p = Props::editing(true);
assert_eq!(p.key, "editing");
```
