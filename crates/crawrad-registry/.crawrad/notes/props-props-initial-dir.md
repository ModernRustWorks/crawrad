---
id: props-props-initial-dir
kind: doc
target_file: src/props.rs
line_hint: 1686
author: 
created: 
title: "Props::initial_dir"
tags: []
signature: "pub fn initial_dir (default : impl Into < String >) -> PropDesc"
---

Initial folder opened by dialog (`InitialDir`).

# Examples

```rust
use crawrad_registry::Props;

let p = Props::initial_dir("C:\\");
assert_eq!(p.key, "initial_dir");
```
