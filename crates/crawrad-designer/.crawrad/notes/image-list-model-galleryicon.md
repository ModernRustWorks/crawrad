---
id: image-list-model-galleryicon
kind: doc
target_file: src/image_list_model.rs
line_hint: 181
author: 
created: 
title: "GalleryIcon"
tags: []
signature: "pub struct GalleryIcon"
---

An icon from the gallery — built-in or from the user.

`String` and not `&'static str` like [`IconPreset`]: what the user
adds is read from a file at runtime, and a `&'static` does not
survive that. The built-in table remains `&'static` in the binary and
is converted here — the cost is one allocation per icon per window open,
over a list of dozens.

# Examples

```rust
use crawrad_designer::image_list_model::GalleryIcon;

let icon = GalleryIcon {
    name: "Save".to_string(),
    symbol: "💾".to_string(),
    category: "File".to_string(),
    builtin: true,
};
assert_eq!(icon.name, "Save");
```
