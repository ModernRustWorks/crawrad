---
id: image-list-model-no-category
kind: doc
target_file: src/image_list_model.rs
line_hint: 376
author: 
created: 
title: "NO_CATEGORY"
tags: []
signature: "pub const NO_CATEGORY : & str = \"(No Category)\""
---

The category for an entry that declared none.

The category for an entry that declared none. It is the label of the classic RAD IDE, and the same one that
ActionList uses.

# Examples

```rust
use crawrad_designer::image_list_model::NO_CATEGORY;

assert_eq!(NO_CATEGORY, "(No Category)");
```
