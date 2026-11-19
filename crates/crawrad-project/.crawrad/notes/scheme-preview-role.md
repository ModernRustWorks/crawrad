---
id: scheme-preview-role
kind: doc
target_file: src/scheme_preview.rs
line_hint: 7
author: 
created: 
title: "Role"
tags: []
signature: "pub enum Role"
---

Syntax and diagnostic token roles for preview spans.

# Examples

```rust
use crawrad_project::scheme_preview::Role;

assert_ne!(Role::Keyword, Role::Type);
```
