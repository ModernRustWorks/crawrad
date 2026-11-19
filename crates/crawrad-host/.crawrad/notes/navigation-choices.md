---
id: navigation-choices
kind: doc
target_file: src/navigation.rs
line_hint: 100
author: 
created: 
title: "Choices"
tags: []
signature: "pub struct Choices"
---

Candidates for ambiguous definition jumps and their return origin.

Several definitions answered: the places to choose from, and where
Go to Definition was asked (Back returns there once one is chosen).

# Examples

```rust
use crawrad_host::navigation::Choices;

let choices = Choices::default();
assert!(!choices.is_open());
```
