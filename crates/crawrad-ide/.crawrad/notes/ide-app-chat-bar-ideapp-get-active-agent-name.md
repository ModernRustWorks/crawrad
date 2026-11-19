---
id: ide-app-chat-bar-ideapp-get-active-agent-name
kind: doc
target_file: src/ide_app/chat_bar.rs
line_hint: 385
author: 
created: 
title: "IdeApp::get_active_agent_name"
tags: []
signature: "pub fn get_active_agent_name (& self) -> String"
---

Returns the active agent name for display in cards.

Returns the friendly name of the active agent/provider for display in response cards.

# Examples

```rust,ignore
// binary crate: doctests cannot reach main.rs items
let app = IdeApp::default();
assert!(!app.get_active_agent_name().is_empty());
```
