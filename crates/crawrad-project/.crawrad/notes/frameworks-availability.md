---
id: frameworks-availability
kind: doc
target_file: src/frameworks.rs
line_hint: 11
author: 
created: 
title: "Availability"
tags: []
signature: "pub enum Availability"
---

Whether a framework can be chosen today.

# Examples

```rust
use crawrad_project::frameworks::Availability;

assert_eq!(Availability::Ready, Availability::Ready);
```
