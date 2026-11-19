---
id: chat-attachment-selectionattachment
kind: doc
target_file: src/chat_attachment.rs
line_hint: 10
author: 
created: 
title: "SelectionAttachment"
tags: []
signature: "pub struct SelectionAttachment"
---

A selection captured when the user asked for it, sent instead of the live one.

# Examples

```rust
use crawrad_agent::SelectionAttachment;

let att = SelectionAttachment {
    file: "src/lib.rs".to_string(),
    start_line: 1,
    end_line: 5,
    text: "fn main() {}".to_string(),
    truncated: false,
    source_hash: 12345,
};
assert_eq!(att.start_line, 1);
```
