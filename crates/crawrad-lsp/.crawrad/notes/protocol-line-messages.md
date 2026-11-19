---
id: protocol-line-messages
kind: doc
target_file: src/protocol.rs
line_hint: 396
author: 
created: 
title: "line_messages"
tags: []
signature: "pub fn line_messages (diagnostics : & [Diagnostic]) -> Vec < (usize , String , u8) >"
---

Extracts diagnostic messages formatted for display on editor lines.

Each diagnostic as the editor writes it after its line: `(line, message,
severity)` — severity 0 an error, 1 a warning, 2 the rest — ordered by
line and then the worst first, so the editor's "first per line" is the
worst one.

A Hint that fades its text (rust-analyzer's `inactive_code`, #730) says
it by the fade alone: written after its line, it read as an error.

# Examples

```rust
use crawrad_lsp::protocol::line_messages;

let msgs = line_messages(&[]);
assert!(msgs.is_empty());
```
