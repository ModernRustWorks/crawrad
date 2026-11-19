---
id: inspector-model-editors-text-unlisted
kind: doc
target_file: src/inspector_model/editors/text.rs
line_hint: 59
author: 
created: 
title: "Unlisted"
tags: []
signature: "pub struct Unlisted"
---

A free-text field with the reason its choices could not be listed.

# Examples

```rust
use crawrad_actions::inspector_model::editor::PropEditor;
use crawrad_actions::inspector_model::editors::text::Unlisted;

let editor = Unlisted::new("name", "Columns not listed: probe missing");
assert_eq!(editor.template(), "RowWhenBad");
```
