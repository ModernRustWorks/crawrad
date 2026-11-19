---
id: props-props-allow-all-up
kind: doc
target_file: src/props.rs
line_hint: 1574
author: 
created: 
title: "Props::allow_all_up"
tags: []
signature: "pub fn allow_all_up (default : bool) -> PropDesc"
---

Allows all group buttons to be unselected (`AllowAllUp`).

# Examples

```rust
use crawrad_registry::Props;

let p = Props::allow_all_up(true);
assert_eq!(p.key, "allow_all_up");
```
