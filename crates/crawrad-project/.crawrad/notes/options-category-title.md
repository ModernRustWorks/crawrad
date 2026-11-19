---
id: options-category-title
kind: doc
target_file: src/options.rs
line_hint: 137
author: 
created: 
title: "Category::title"
tags: []
signature: "pub fn title (self) -> & 'static str"
---

The TITLE of the page, which is not always the tree label.

`Editor > General` becomes `Editor` on the page — that is what the egui shell shows, and
is what the classic RAD IDE does: the tree says where you are, the title says what
you are seeing.

# Examples

```rust
use crawrad_project::options::Category;

assert_eq!(Category::Editor.title(), "Editor");
```
