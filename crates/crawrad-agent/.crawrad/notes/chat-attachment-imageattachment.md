---
id: chat-attachment-imageattachment
kind: doc
target_file: src/chat_attachment.rs
line_hint: 110
author: 
created: 
title: "ImageAttachment"
tags: []
signature: "pub struct ImageAttachment"
---

A picture pasted into the chat: the PNG saved on disk and its size.

The agent receives it as an image block when it declares
`promptCapabilities.image`; otherwise it receives the file's path.

# Examples

```rust
use crawrad_agent::ImageAttachment;

let img = ImageAttachment { path: "shot.png".into(), width: 800, height: 600 };
assert!(img.chip_text(0).ends_with("600"));
```
