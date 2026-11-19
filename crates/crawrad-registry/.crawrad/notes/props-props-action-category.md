---
id: props-props-action-category
kind: doc
target_file: src/props.rs
line_hint: 1038
author: 
created: 
title: "Props::action_category"
tags: []
signature: "pub fn action_category (default : & str) -> PropDesc"
---

Action category (`Category`).

# Examples

```rust
use crawrad_registry::Props;

let p = Props::action_category("File");
assert_eq!(p.key, "category");
```
