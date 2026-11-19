---
id: props-props-group-bar
kind: doc
target_file: src/props.rs
line_hint: 1824
author: 
created: 
title: "Props::group_bar"
tags: []
signature: "pub fn group_bar (default : bool) -> PropDesc"
---

Shows the "Group by" bar of a bound Grid (`GroupBar`). Off by default.

# Examples

```rust
use crawrad_registry::Props;

let p = Props::group_bar(true);
assert_eq!(p.key, "group_bar");
```
