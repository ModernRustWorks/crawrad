---
id: editor-chords-reference-markdown
kind: doc
target_file: src/editor_chords.rs
line_hint: 170
author: 
created: 
title: "shortcuts_reference_markdown"
tags: []
signature: "pub fn shortcuts_reference_markdown () -> String"
---

The manual page listing the code editor's keyboard shortcuts.

# Examples

```rust
let md = crawrad_host::editor_chords::shortcuts_reference_markdown();
assert!(md.starts_with("# Keyboard shortcuts"));
```
