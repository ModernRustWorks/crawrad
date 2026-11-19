---
id: options-category-label
kind: doc
target_file: src/options.rs
line_hint: 119
author: 
created: 
title: "Category::label"
tags: []
signature: "pub fn label (self) -> & 'static str"
---

What is read in the TREE.

# Examples

```rust
use crawrad_project::options::Category;

assert_eq!(Category::CommonlyUsed.label(), "Commonly Used");
```
