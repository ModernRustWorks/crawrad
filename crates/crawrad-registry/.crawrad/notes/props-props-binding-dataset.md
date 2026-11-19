---
id: props-props-binding-dataset
kind: doc
target_file: src/props.rs
line_hint: 446
author: 
created: 
title: "Props::binding_dataset"
tags: []
signature: "pub fn binding_dataset () -> PropDesc"
---

Reference to a dataset for the Inspector's Data category (#788 slice 3a).

# Examples

```rust
use crawrad_registry::Props;

let p = Props::binding_dataset();
assert_eq!(p.key, "binding.dataset");
```
