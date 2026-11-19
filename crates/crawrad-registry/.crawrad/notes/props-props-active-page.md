---
id: props-props-active-page
kind: doc
target_file: src/props.rs
line_hint: 838
author: 
created: 
title: "Props::active_page"
tags: []
signature: "pub fn active_page (default : i64) -> PropDesc"
---

Active page index in a `PageControl` (`ActivePageIndex`).

# Examples

```rust
use crawrad_registry::Props;

let p = Props::active_page(0);
assert_eq!(p.key, "active_page");
```
