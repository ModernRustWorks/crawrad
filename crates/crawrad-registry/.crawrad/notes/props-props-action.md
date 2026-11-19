---
id: props-props-action
kind: doc
target_file: src/props.rs
line_hint: 422
author: 
created: 
title: "Props::action"
tags: []
signature: "pub fn action () -> PropDesc"
---

Action bound to the component (`Action`).

# Examples

```rust
use crawrad_registry::Props;

let p = Props::action();
assert_eq!(p.key, "action");
```
