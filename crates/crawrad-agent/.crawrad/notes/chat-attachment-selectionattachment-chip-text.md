---
id: chat-attachment-selectionattachment-chip-text
kind: doc
target_file: src/chat_attachment.rs
line_hint: 60
author: 
created: 
title: "SelectionAttachment::chip_text"
tags: []
signature: "pub fn chip_text (& self , current_file_text : Option < & str >) -> String"
---

Formats the attachment as a chip text.

Formats the attachment as a chip text with optional truncated/changed indicators.

# Examples

```rust
use crawrad_agent::SelectionAttachment;

let att = SelectionAttachment {
    file: "src/lib.rs".to_string(),
    start_line: 1,
    end_line: 5,
    text: "fn main() {}".to_string(),
    truncated: false,
    source_hash: 0,
};
assert_eq!(att.chip_text(None), "lib.rs:1-5");
```
