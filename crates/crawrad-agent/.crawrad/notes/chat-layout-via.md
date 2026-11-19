---
id: chat-layout-via
kind: doc
target_file: src/chat_layout.rs
line_hint: 405
author: 
created: 
title: "Via"
tags: []
signature: "pub enum Via"
---

Indicates the transport delivery mechanism used for a chat reply.

How a reply reached the chat: through ACP (with its session) or through
the agent's CLI print mode, the fallback when ACP could not be used.

# Examples

```rust
use crawrad_agent::chat_layout::Via;

let via = Via::Acp { session: Some("s1".to_string()) };
assert!(matches!(via, Via::Acp { .. }));
```
