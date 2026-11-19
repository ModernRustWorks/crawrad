---
id: chat-attachment-selectionattachment-capture
kind: doc
target_file: src/chat_attachment.rs
line_hint: 33
author: 
created: 
title: "SelectionAttachment::capture"
tags: []
signature: "pub fn capture (file : & str , selection : & EditorSelectionContext , file_text : & str , max_bytes : usize ,) -> Self"
---

Captures a snapshot of the selection from the active file buffer.

# Examples

```rust
use crawrad_agent::prompt_assembler::EditorSelectionContext;
use crawrad_agent::SelectionAttachment;

let sel = EditorSelectionContext {
    start_line: 1,
    end_line: 1,
    selected_text: "println!(\"hi\");".to_string(),
};
let att = SelectionAttachment::capture("src/main.rs", &sel, "println!(\"hi\");", 100);
assert_eq!(att.start_line, 1);
assert!(!att.truncated);
```
