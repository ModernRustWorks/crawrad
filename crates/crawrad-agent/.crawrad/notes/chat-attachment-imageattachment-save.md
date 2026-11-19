---
id: chat-attachment-imageattachment-save
kind: doc
target_file: src/chat_attachment.rs
line_hint: 118
author: 
created: 
title: "ImageAttachment::save"
tags: []
signature: "pub fn save (dir : & Path , png : & [u8]) -> Result < Self , String >"
---

Saves the PNG to `dir` as `image-<n>.png` and reads its size.

`n` is the first free number.

# Errors

When the bytes are not a PNG or the file cannot be written.

# Examples

```rust,no_run
use crawrad_agent::{ImageAttachment, images_dir};

let png = std::fs::read("shot.png").unwrap();
let img = ImageAttachment::save(&images_dir(), &png).unwrap();
println!("{}x{}", img.width, img.height);
```
