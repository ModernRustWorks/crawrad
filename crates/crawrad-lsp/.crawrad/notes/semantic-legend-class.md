---
id: semantic-legend-class
kind: doc
target_file: src/semantic.rs
line_hint: 82
author: 
created: 
title: "Legend::class"
tags: []
signature: "pub fn class (& self , ty : u32 , mods : u32) -> Option < Class >"
---

Classifies a token type index and modifier bitmask.

The class of a token of type index `ty` with the modifier bits
`mods`, or `None` when the tokenizer's colour is enough.

# Examples

```rust
use crawrad_lsp::semantic::{Class, Legend};

let legend = Legend {
    types: vec!["macro".to_string()],
    modifiers: Vec::new(),
};
assert_eq!(legend.class(0, 0), Some(Class::Macro));
```
