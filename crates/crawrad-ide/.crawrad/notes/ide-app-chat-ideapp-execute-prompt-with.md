---
id: ide-app-chat-ideapp-execute-prompt-with
kind: doc
target_file: src/ide_app/chat.rs
line_hint: 702
author: 
created: 
title: "IdeApp::execute_prompt_with"
tags: []
signature: "pub fn execute_prompt_with (& mut self , cx : & mut Cx , prompt : & str , attachments : Vec < SelectionAttachment > ,)"
---

Sends `prompt` with these attachments (a queued message carries its own).

# Examples

```rust,ignore
// binary crate: doctests cannot reach main.rs items
let mut app = IdeApp::default();
assert!(!app.chat.busy);
```
