---
id: props-props-badge-kind
kind: doc
target_file: src/props.rs
line_hint: 2003
author: 
created: 
title: "Props::badge_kind"
tags: []
signature: "pub fn badge_kind (default : & str) -> PropDesc"
---

Classification kind / semantic tone of a `Badge` (`Kind`, #500).

# Examples

```rust
use crawrad_registry::Props;

let p = Props::badge_kind("bkPrimary");
assert_eq!(p.key, "kind");
```
