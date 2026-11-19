---
id: props-props-text-layout
kind: doc
target_file: src/props.rs
line_hint: 1237
author: 
created: 
title: "Props::text_layout"
tags: []
signature: "pub fn text_layout (default : & 'static str) -> PropDesc"
---

Where the text sits down the control (`Layout`).

# Examples

```rust
use crawrad_registry::Props;

let p = Props::text_layout("tlTop");
assert_eq!(p.key, "layout");
```
