---
id: components-image-list-desenho
kind: doc
target_file: src/components/image_list.rs
line_hint: 9
author: 
created: 
title: "DESENHO"
tags: []
signature: "pub static DESENHO : Badge = Badge :: for_class (\"ImageList\")"
---

O desenho do `ImageList`.

# Examples

```rust
use crawrad_runtime_makepad::components::ComponentDrawing;
use crawrad_runtime_makepad::components::image_list::DESENHO;

assert_eq!(DESENHO.class_name(), "ImageList");
```
