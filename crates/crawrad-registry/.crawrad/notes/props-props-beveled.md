---
id: props-props-beveled
kind: doc
target_file: src/props.rs
line_hint: 1891
author: 
created: 
title: "Props::beveled"
tags: []
signature: "pub fn beveled (default : bool) -> PropDesc"
---

Beveled drawing / grip marks on Splitter (`Beveled`).

# Examples

```rust
use crawrad_registry::Props;

let p = Props::beveled(true);
assert_eq!(p.key, "beveled");
```
