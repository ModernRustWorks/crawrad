---
id: props-props-tab-position
kind: doc
target_file: src/props.rs
line_hint: 849
author: 
created: 
title: "Props::tab_position"
tags: []
signature: "pub fn tab_position (default : & 'static str) -> PropDesc"
---

Where a page control's tabs sit (`TabPosition`, #449).

# Examples

```rust
use crawrad_registry::Props;

let p = Props::tab_position("tpTop");
assert_eq!(p.key, "tab_position");
```
