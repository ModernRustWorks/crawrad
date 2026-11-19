---
id: components-image-list-image-list
kind: doc
target_file: src/components/image_list.rs
line_hint: 4
author: 
created: 
title: "image_list"
tags: []
signature: "pub fn image_list () -> ComponentDesc"
---

`ImageList`: icon collection referenced by index in buttons, actions, and menus.

# Examples

```rust
use crawrad_registry::image_list;

let desc = image_list();
assert_eq!(desc.class, "ImageList");
```
