---
id: edit-apply-text-edits
kind: doc
target_file: src/edit.rs
line_hint: 57
author: 
created: 
title: "apply_text_edits"
tags: []
signature: "pub fn apply_text_edits (text : & str , edits : & [TextEdit] , enc : crate :: encoding :: PositionEncoding ,) -> String"
---

Applies a sequence of non-overlapping text edits to `text`.

`text` with `edits` applied. The edits are the server's for this text:
they do not overlap and their positions refer to the text before any of
them, so they apply from the last to the first.

# Examples

```rust
use crawrad_lsp::edit::{apply_text_edits, TextEdit};
use crawrad_lsp::encoding::PositionEncoding;
use crawrad_lsp::protocol::{Position, Range};

let edits = vec![TextEdit {
    range: Range::new(Position::new(0, 0), Position::new(0, 5)),
    new_text: "world".to_string(),
}];
let result = apply_text_edits("hello there", &edits, PositionEncoding::Utf8);
assert_eq!(result, "world there");
```
