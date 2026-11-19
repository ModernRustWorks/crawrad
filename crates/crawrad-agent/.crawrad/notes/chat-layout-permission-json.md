---
id: chat-layout-permission-json
kind: doc
target_file: src/chat_layout.rs
line_hint: 240
author: 
created: 
title: "permission_json"
tags: []
signature: "pub fn permission_json (queue : & crate :: permission :: PermissionQueue) -> serde_json :: Value"
---

Formats the permission card status for view reporting.

The permission card as `view.get_chat` reports it: the request on top and how many wait.

# Examples

```rust
use crawrad_agent::chat_layout::permission_json;
use crawrad_agent::PermissionQueue;

let queue = PermissionQueue::new();
assert!(permission_json(&queue).is_null());
```
