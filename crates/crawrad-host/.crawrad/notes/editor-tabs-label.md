---
id: editor-tabs-label
kind: doc
target_file: src/editor_tabs.rs
line_hint: 33
author: 
created: 
title: "Label"
tags: []
signature: "pub struct Label"
---

What one tab writes on itself.

# Examples

```rust
use crawrad_host::editor_tabs::Label;

let label = Label { name: "main.rs".into(), hint: None };
assert_eq!(label.one_line(), "main.rs");
```
