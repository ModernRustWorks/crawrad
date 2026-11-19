---
id: edit-selection-range
kind: doc
target_file: src/edit.rs
line_hint: 62
author: 
created: 
title: "selection_range"
tags: []
signature: "pub fn selection_range (text : & str , ends : [(usize , usize) ; 2]) -> Value"
---

Converts selection endpoints into 1-based line and column positions.

`editor.get_selection`'s `range`: the selection's `[(line, byte)]` ends
(0-based, as the editor keeps them) as 1-based lines and 1-based columns
counted in characters, the numbering `editor.apply_edit` takes.

# Examples

```rust
use crawrad_mcp::edit::selection_range;

let text = "hello\nworld\n";
let range = selection_range(text, [(0, 1), (1, 3)]);
assert_eq!(range["start"]["line"], 1);
assert_eq!(range["start"]["column"], 2);
```
