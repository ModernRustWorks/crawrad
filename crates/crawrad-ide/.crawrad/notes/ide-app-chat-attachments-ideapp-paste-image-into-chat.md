---
id: ide-app-chat-attachments-ideapp-paste-image-into-chat
kind: doc
target_file: src/ide_app/chat_attachments.rs
line_hint: 79
author: 
created: 
title: "IdeApp::paste_image_into_chat"
tags: []
signature: "pub fn paste_image_into_chat (& mut self , cx : & mut Cx) -> bool"
---

Ctrl+V in the chat prompt: a picture on the clipboard becomes an image chip.

Returns false when the clipboard holds text (the text paste goes on as
always) or no picture.

# Examples

```rust,ignore
if self.chat_prompt_focused(cx) && self.paste_image_into_chat(cx) {
    return;
}
```
