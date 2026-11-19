---
id: chat-attachment-imageattachment-chip-text
kind: doc
target_file: src/chat_attachment.rs
line_hint: 150
author: 
created: 
title: "ImageAttachment::chip_text"
tags: []
signature: "pub fn chip_text (& self , index : usize) -> String"
---

The chip's text: `Image <n>` and the size, `n` counting from 1.

# Examples

```rust
use crawrad_agent::ImageAttachment;

let img = ImageAttachment { path: "a.png".into(), width: 2, height: 1 };
assert!(img.chip_text(1).starts_with("Image 2"));
```
