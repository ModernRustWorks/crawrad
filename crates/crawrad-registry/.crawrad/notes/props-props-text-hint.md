---
id: props-props-text-hint
kind: doc
target_file: src/props.rs
line_hint: 2094
author: 
created: 
title: "Props::text_hint"
tags: []
signature: "pub fn text_hint (default : & str) -> PropDesc"
---

Placeholder text when nothing is selected (`TextHint`, #500).

# Examples

```rust
use crawrad_registry::Props;

let p = Props::text_hint("Select an item...");
assert_eq!(p.key, "text_hint");
```
