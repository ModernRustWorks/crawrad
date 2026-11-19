---
id: props-props-num-glyphs
kind: doc
target_file: src/props.rs
line_hint: 1719
author: 
created: 
title: "Props::num_glyphs"
tags: []
signature: "pub fn num_glyphs (default : i64) -> PropDesc"
---

Number of glyphs in button bitmap (`NumGlyphs`).

# Examples

```rust
use crawrad_registry::Props;

let p = Props::num_glyphs(1);
assert_eq!(p.key, "num_glyphs");
```
