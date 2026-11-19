---
id: ide-app-chat-ideapp-answer-permission
kind: doc
target_file: src/ide_app/chat.rs
line_hint: 646
author: 
created: 
title: "IdeApp::answer_permission"
tags: []
signature: "pub fn answer_permission (& mut self , cx : & mut Cx , choice : & str)"
---

The permission card's answer (#420): the click and `chat.permission {choice}`.

# Examples

```rust,ignore
// binary crate: doctests cannot reach main.rs items
let mut app = IdeApp::default();
assert!(app.chat.permission_queue.is_empty());
```

# Panics

Panics if the session permissions mutex is poisoned.
