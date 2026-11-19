---
id: chat-layout-delivery-word
kind: doc
target_file: src/chat_layout.rs
line_hint: 37
author: 
created: 
title: "delivery_word"
tags: []
signature: "pub fn delivery_word (reply_started : bool) -> & 'static str"
---

Returns the plain text delivery mark characters for test or MCP display.

Returns the plain text delivery mark characters for tests / MCP JSON representation.

Returns `"✓"` when sent (not yet replied),
and `"✓✓"` when the agent reply has started.

# Examples

```rust
use crawrad_agent::chat_layout::delivery_word;

assert_eq!(delivery_word(false), "✓");
assert_eq!(delivery_word(true), "✓✓");
```
