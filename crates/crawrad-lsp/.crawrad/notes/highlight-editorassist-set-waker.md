---
id: highlight-editorassist-set-waker
kind: doc
target_file: src/highlight.rs
line_hint: 224
author: 
created: 
title: "EditorAssist::set_waker"
tags: []
signature: "pub fn set_waker (& self , wake : impl Fn () + Send + Sync + Clone + 'static)"
---

What wakes the window when either answers.

# Examples

```rust
use crawrad_lsp::highlight::EditorAssist;

let assist = EditorAssist::default();
assist.set_waker(|| {});
assert!(assist.hints.take().is_none());
```
