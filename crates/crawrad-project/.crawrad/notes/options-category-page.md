---
id: options-category-page
kind: doc
target_file: src/options.rs
line_hint: 81
author: 
created: 
title: "Category::page"
tags: []
signature: "pub fn page (self) -> Option < & 'static str >"
---

Returns the page that shows this category, one of [`Self::PAGES`].

The page that shows it, one of [`Self::PAGES`]; the two AI entries
share one, and a page that only informs has none.

# Examples

```rust
use crawrad_project::options::Category;

assert_eq!(Category::CommonlyUsed.page(), Some("opt_commonly_used"));
assert_eq!(Category::IdeEnvironmentVariables.page(), None);
```
