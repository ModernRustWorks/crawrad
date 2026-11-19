---
id: editor-tabs-label-one-line
kind: doc
target_file: src/editor_tabs.rs
line_hint: 48
author: 
created: 
title: "Label::one_line"
tags: []
signature: "pub fn one_line (& self) -> String"
---

The tab as one line — what a widget with a single text slot shows.

# Examples

```rust
use crawrad_host::editor_tabs::Label;

let label = Label { name: "lib.rs".into(), hint: Some("src".into()) };
assert_eq!(label.one_line(), "lib.rs  src");
```
