---
id: image-list-model-icon-refusal
kind: doc
target_file: src/image_list_model.rs
line_hint: 240
author: 
created: 
title: "icon_refusal"
tags: []
signature: "pub fn icon_refusal (name : & str , symbol : & str , already_exists : & dyn Fn (& str) -> bool ,) -> Option < & 'static str >"
---

Validates if an icon can enter the gallery.

What a new icon needs to enter the gallery — and what is missing, when
it is missing.

Three rules, and all three are visible on screen:

1. **non-empty name**, because it is what the form references;
2. **non-empty symbol**, because it is what draws;
3. **free name**, because two icons with the same name would make the second
   never be chosen.

The category may be missing: it only groups, and the classic RAD IDE's `(No Category)` exists
exactly for that.

# Examples

```rust
use crawrad_designer::image_list_model::icon_refusal;

assert!(icon_refusal("", "💾", &|_| false).is_some());
assert!(icon_refusal("Save", "💾", &|_| false).is_none());
```
