---
id: options-category-note
kind: doc
target_file: src/options.rs
line_hint: 99
author: 
created: 
title: "Category::note"
tags: []
signature: "pub fn note (self) -> Option < & 'static str >"
---

What a page that only INFORMS says.

`None` for the pages with real controls: there the sentence would be a
caption of nothing. Moved out of the Makepad shell, which only shows it.

# Examples

```rust
use crawrad_project::options::Category;

assert!(Category::IdeEnvironmentVariables.note().is_some());
assert_eq!(Category::CommonlyUsed.note(), None);
```
