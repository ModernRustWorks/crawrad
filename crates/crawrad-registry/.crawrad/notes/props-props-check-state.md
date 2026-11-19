---
id: props-props-check-state
kind: doc
target_file: src/props.rs
line_hint: 323
author: 
created: 
title: "Props::check_state"
tags: []
signature: "pub fn check_state (default : & 'static str) -> PropDesc"
---

A check box's three-state value (`State`, #445).

A check box's three states (`State`, #445): `cbGrayed` is "neither",
the indeterminate mark. `Checked` and `State` are kept in agreement by
`component.set_property`.

# Examples

```rust
use crawrad_registry::Props;

let p = Props::check_state("cbChecked");
assert_eq!(p.key, "state");
```
