---
id: line-edits-apply-line-edit
kind: doc
target_file: src/line_edits.rs
line_hint: 163
author: 
created: 
title: "apply_line_edit"
tags: []
signature: "pub fn apply_line_edit (command : Command , lines : & [String] , start_line : usize , end_line : usize ,) -> Option < (Vec < String > , usize) >"
---

Dispatches a whole-line editing command across the given line range.

Dispatches whole-line editing commands (`ToggleComment`, `MoveLines`,
`DuplicateLines`, `DeleteLine`) returning the updated lines and the new
cursor line index.

# Examples

```rust
use crawrad_host::editor_chords::Command;
use crawrad_host::line_edits::apply_line_edit;

let lines = vec!["hello".into()];
let res = apply_line_edit(Command::ToggleComment, &lines, 0, 0);
assert!(res.is_some());
```
