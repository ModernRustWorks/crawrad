---
id: props-props-alignment
kind: doc
target_file: src/props.rs
line_hint: 1213
author: 
created: 
title: "Props::alignment"
tags: []
signature: "pub fn alignment (default : & 'static str) -> PropDesc"
---

Where the text sits across the control (`Alignment`).

# Examples

```rust
use crawrad_registry::Props;

let p = Props::alignment("taLeftJustify");
assert_eq!(p.key, "alignment");
```
