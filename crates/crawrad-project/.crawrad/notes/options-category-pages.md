---
id: options-category-pages
kind: doc
target_file: src/options.rs
line_hint: 67
author: 
created: 
title: "Category::PAGES"
tags: []
signature: "pub const PAGES : [& 'static str ; 10] = [\"opt_commonly_used\" , \"opt_folders\" , \"opt_history\" , \"opt_saving\" , \"opt_designer\" , \"opt_appearance\" , \"opt_editor\" , \"opt_color_scheme\" , \"opt_keyboard\" , \"opt_ai\" ,]"
---

Every page of the dialog, by its DSL name, once.

# Examples

```rust
use crawrad_project::options::Category;

assert_eq!(Category::PAGES.len(), 10);
```
