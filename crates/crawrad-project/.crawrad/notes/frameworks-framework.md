---
id: frameworks-framework
kind: doc
target_file: src/frameworks.rs
line_hint: 20
author: 
created: 
title: "Framework"
tags: []
signature: "pub struct Framework"
---

One framework, as the dialog shows it.

# Examples

```rust
use crawrad_project::frameworks::{CATALOGUE, Framework};

let fw: &Framework = &CATALOGUE[0];
assert_eq!(fw.name, "Makepad");
```
