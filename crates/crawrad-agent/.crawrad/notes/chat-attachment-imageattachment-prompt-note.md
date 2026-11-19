---
id: chat-attachment-imageattachment-prompt-note
kind: doc
target_file: src/chat_attachment.rs
line_hint: 167
author: 
created: 
title: "ImageAttachment::prompt_note"
tags: []
signature: "pub fn prompt_note (& self) -> String"
---

The line sent instead of the picture to an agent that takes no images: its path.

# Examples

```rust
use crawrad_agent::ImageAttachment;

let img = ImageAttachment { path: "shot.png".into(), width: 1, height: 1 };
assert!(img.prompt_note().contains("shot.png"));
```
