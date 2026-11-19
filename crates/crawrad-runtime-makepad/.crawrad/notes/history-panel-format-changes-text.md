---
id: history-panel-format-changes-text
kind: doc
target_file: src/history_panel.rs
line_hint: 333
author: 
created: 
title: "format_changes_text"
tags: []
signature: "pub fn format_changes_text (blocks : usize , current_block : Option < usize >) -> String"
---

Formats the changes indicator text for the differences bar.

# Examples

```rust
use crawrad_runtime_makepad::history_panel::format_changes_text;

assert_eq!(format_changes_text(0, None), "No changes");
assert_eq!(format_changes_text(2, Some(0)), "2 changes · block 1 of 2");
```
