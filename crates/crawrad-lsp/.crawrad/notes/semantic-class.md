---
id: semantic-class
kind: doc
target_file: src/semantic.rs
line_hint: 27
author: 
created: 
title: "Class"
tags: []
signature: "pub enum Class"
---

Semantic token classes that the editor colours.

The classes the editor colours, in the order of the editor's own
`SemanticKind::ALL`.

# Examples

```rust
use crawrad_lsp::semantic::Class;

assert_eq!(Class::Macro as u8, 0);
```
