---
id: chat-attachment-prompt-blocks
kind: doc
target_file: src/chat_attachment.rs
line_hint: 181
author: 
created: 
title: "prompt_blocks"
tags: []
signature: "pub fn prompt_blocks (prompt_text : & str , images : & [ImageAttachment] , accepts_images : bool) -> Vec < serde_json :: Value >"
---

The `session/prompt` content: the text, then one block per picture.

A picture is an image block when the agent accepts images and the file
reads; otherwise a text block with its path.

# Examples

```rust
use crawrad_agent::{ImageAttachment, prompt_blocks};

let img = ImageAttachment { path: "missing.png".into(), width: 1, height: 1 };
let blocks = prompt_blocks("why?", &[img], false);
assert_eq!(blocks.len(), 2);
assert_eq!(blocks[1]["type"], "text");
```
