---
id: props-props-collapsed
kind: doc
target_file: src/props.rs
line_hint: 1093
author: 
created: 
title: "Props::collapsed"
tags: []
signature: "pub fn collapsed (default : bool) -> PropDesc"
---

Whether a `CategoryPanel` is folded to its header (#500).

# Examples

```rust
use crawrad_registry::Props;

let p = Props::collapsed(false);
assert_eq!(p.key, "collapsed");
```
