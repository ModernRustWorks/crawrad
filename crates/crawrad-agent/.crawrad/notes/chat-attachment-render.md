---
id: chat-attachment-render
kind: doc
target_file: src/chat_attachment.rs
line_hint: 81
author: 
created: 
title: "render"
tags: []
signature: "pub fn render (attachments : & [SelectionAttachment]) -> String"
---

Renders the prompt block for attachments to be placed before the user query.

# Examples

```rust
use crawrad_agent::chat_attachment::{SelectionAttachment, render};

let att = SelectionAttachment {
    file: "src/lib.rs".to_string(),
    start_line: 1,
    end_line: 5,
    text: "fn main() {}".to_string(),
    truncated: false,
    source_hash: 0,
};
let text = render(&[att]);
assert!(text.contains("Attached selection"));
```
