---
id: chat-attachment-images-dir
kind: doc
target_file: src/chat_attachment.rs
line_hint: 176
author: 
created: 
title: "images_dir"
tags: []
signature: "pub fn images_dir () -> PathBuf"
---

Where pasted pictures are saved: `crawrad-chat-images` in the temp folder.

# Examples

```rust
assert!(crawrad_agent::images_dir().ends_with("crawrad-chat-images"));
```
