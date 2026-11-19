---
id: chat-attachment-imageattachment-acp-block
kind: doc
target_file: src/chat_attachment.rs
line_hint: 155
author: 
created: 
title: "ImageAttachment::acp_block"
tags: []
signature: "pub fn acp_block (& self) -> Result < serde_json :: Value , String >"
---

The picture as an ACP image content block (`image/png`, base64 data).

# Errors

When the file cannot be read.

# Examples

```rust,no_run
use crawrad_agent::ImageAttachment;

let img = ImageAttachment { path: "shot.png".into(), width: 1, height: 1 };
let block = img.acp_block().unwrap();
assert_eq!(block["type"], "image");
```
