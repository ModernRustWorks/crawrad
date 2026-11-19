---
id: editor-footer-editor-footer-cells
kind: doc
target_file: src/editor_footer.rs
line_hint: 59
author: 
created: 
title: "editor_footer_cells"
tags: []
signature: "pub fn editor_footer_cells (line : usize , byte : usize , line_text : & str , editable : bool , modified : bool , encoding : & str , language : & str ,) -> Vec < String >"
---

Returns the formatted cells of the editor footer in slot order.

The cells of the editor footer in slot order:
1. Caret position (`Ln X, Col Y`)
2. Mode (`Insert` or `Read only`)
3. Modified indicator (`Modified` or empty)
4. Indentation (`Spaces: 4`)
5. Encoding
6. Language name

# Examples

```rust
use crawrad_host::editor_footer::editor_footer_cells;

let cells = editor_footer_cells(0, 0, "", true, false, "UTF-8", "Rust");
assert_eq!(cells.len(), 6);
assert_eq!(cells[0], "Ln 1, Col 1");
```
