---
id: ide-app-chat-attachments-ideapp-render-attachment-chips
kind: doc
target_file: src/ide_app/chat_attachments.rs
line_hint: 78
author: 
created: 
title: "IdeApp::render_attachment_chips"
tags: []
signature: "pub fn render_attachment_chips (& mut self , cx : & mut Cx)"
---

One chip per attachment above the prompt; the row hides when there is none.

# Examples

```rust,ignore
// binary crate: doctests cannot reach main.rs items
let mut app = IdeApp::default();
assert!(app.chat.attachments.is_empty());
```
