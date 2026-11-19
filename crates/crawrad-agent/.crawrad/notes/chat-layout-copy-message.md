---
id: chat-layout-copy-message
kind: doc
target_file: src/chat_layout.rs
line_hint: 155
author: 
created: 
title: "copy_message"
tags: []
signature: "pub fn copy_message (messages : & [crate :: session :: ChatMessage] , default_sender : & str , args : & serde_json :: Value ,) -> Result < String , String >"
---

Copies text from a specified chat turn message part.

Groups chat session messages into sequential conversational turns.
What `chat.copy_message {turn, part}` copies: turn `turn` of the whole
conversation (0 = the oldest), its `part` `"user"` or `"reply"` (the default,
the reply's whole markdown).

# Errors

Returns an error if the requested turn or message part cannot be found.

# Examples

```rust
use crawrad_agent::chat_layout::copy_message;
use serde_json::json;

let res = copy_message(&[], "Claude", &json!({"turn": 0}));
assert!(res.is_err());
```
