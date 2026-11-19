---
id: props-props-enabled
kind: doc
target_file: src/props.rs
line_hint: 175
author: 
created: 
title: "Props::enabled"
tags: []
signature: "pub fn enabled (default : bool) -> PropDesc"
---

Component enabled for interaction (`Enabled`).

# Examples

```rust
use crawrad_registry::Props;

let p = Props::enabled(true);
assert_eq!(p.key, "enabled");
```
