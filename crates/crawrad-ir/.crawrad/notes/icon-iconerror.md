---
id: icon-iconerror
kind: doc
target_file: src/icon.rs
line_hint: 39
author: 
created: 
title: "IconError"
tags: []
signature: "pub enum IconError"
---

Why a declared icon is refused.

# Examples

```rust
use crawrad_ir::icon::{validate, IconError};

assert_eq!(validate(&[]), Err(IconError::Empty));
```
