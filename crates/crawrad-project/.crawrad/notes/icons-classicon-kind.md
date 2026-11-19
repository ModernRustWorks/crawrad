---
id: icons-classicon-kind
kind: doc
target_file: src/icons.rs
line_hint: 551
author: 
created: 
title: "ClassIcon::kind"
tags: []
signature: "pub fn kind (self) -> f64"
---

The value the shader switches on.

Derived from the position in [`Self::ALL`] so that a new icon cannot be
given a number that is already taken. The shader reads it as a float and
compares with a tolerance, because that is what a GLSL uniform can do.

# Panics

Panics if the variant is not present in [`ClassIcon::ALL`].

# Examples

```rust
use crawrad_project::icons::ClassIcon;

assert!(ClassIcon::Button.kind() >= 0.0);
```
