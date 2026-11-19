---
id: scheme-preview-linekind
kind: doc
target_file: src/scheme_preview.rs
line_hint: 34
author: 
created: 
title: "LineKind"
tags: []
signature: "pub enum LineKind"
---

Severity classification of a preview line.

# Examples

```rust
use crawrad_project::scheme_preview::LineKind;

assert_ne!(LineKind::Normal, LineKind::Error);
```
