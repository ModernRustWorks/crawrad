---
id: code-lens-lenskind
kind: doc
target_file: src/code_lens.rs
line_hint: 62
author: 
created: 
title: "LensKind"
tags: []
signature: "pub enum LensKind"
---

The action a lens performs.

# Examples

```rust
use crawrad_lsp::code_lens::LensKind;

let kind = LensKind::Run;
assert_eq!(kind, LensKind::Run);
```
