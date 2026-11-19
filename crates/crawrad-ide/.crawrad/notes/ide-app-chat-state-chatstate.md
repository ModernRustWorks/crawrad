---
id: ide-app-chat-state-chatstate
kind: doc
target_file: src/ide_app/chat_state.rs
line_hint: 10
author: 
created: 
title: "ChatState"
tags: []
signature: "pub struct ChatState"
---

The state of AI chat, audio recording, and ACP integration (ADR 0043, PR 7).

# Examples

```rust,ignore
// binary crate: doctests cannot reach main.rs items
let state = ChatState::default();
assert!(!state.busy);
```
