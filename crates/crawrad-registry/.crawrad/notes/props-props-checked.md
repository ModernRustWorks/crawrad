---
id: props-props-checked
kind: doc
target_file: src/props.rs
line_hint: 240
author: 
created: 
title: "Props::checked"
tags: []
signature: "pub fn checked (default : bool) -> PropDesc"
---

Checked state of check boxes and radio buttons (`Checked`).

# Examples

```rust
use crawrad_registry::Props;

let p = Props::checked(true);
assert_eq!(p.key, "checked");
```
