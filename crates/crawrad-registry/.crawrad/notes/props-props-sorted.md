---
id: props-props-sorted
kind: doc
target_file: src/props.rs
line_hint: 816
author: 
created: 
title: "Props::sorted"
tags: []
signature: "pub fn sorted (default : bool) -> PropDesc"
---

Whether a list keeps items alphabetically sorted (`Sorted`, #439).

Whether a list keeps its items in alphabetical order, ignoring case
(`Sorted`, #439).

# Examples

```rust
use crawrad_registry::Props;

let p = Props::sorted(true);
assert_eq!(p.key, "sorted");
```
