---
id: editortext
kind: doc
target_file: src/lib.rs
line_hint: 166
author: 
created: 
title: "EditorText"
tags: []
signature: "pub struct EditorText"
---

What the editor's text actions need to know (#257).

# Examples

```rust
use crawrad_actions::EditorText;

let text = EditorText::default();
assert!(!text.selection);
```
