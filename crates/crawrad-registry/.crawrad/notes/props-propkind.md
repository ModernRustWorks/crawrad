---
id: props-propkind
kind: doc
target_file: src/props.rs
line_hint: 5
author: 
created: 
title: "PropKind"
tags: []
signature: "pub enum PropKind"
---

The visual and semantic nature of a property.

Allows the Object Inspector to draw the correct editing widget without
ad-hoc branching, and guides code generators (Makepad and Egui).

# Examples

```rust
use crawrad_registry::PropKind;

assert_eq!(PropKind::String, PropKind::String);
```
