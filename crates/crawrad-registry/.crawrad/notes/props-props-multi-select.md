---
id: props-props-multi-select
kind: doc
target_file: src/props.rs
line_hint: 805
author: 
created: 
title: "Props::multi_select"
tags: []
signature: "pub fn multi_select (default : bool) -> PropDesc"
---

Whether a list allows multiple row selection (`MultiSelect`, #439).

Whether a list marks several rows: Ctrl+click toggles one, Shift+click
marks a run (`MultiSelect`, #439).

# Examples

```rust
use crawrad_registry::Props;

let p = Props::multi_select(true);
assert_eq!(p.key, "multi_select");
```
