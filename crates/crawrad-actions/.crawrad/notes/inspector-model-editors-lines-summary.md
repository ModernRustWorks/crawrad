---
id: inspector-model-editors-lines-summary
kind: doc
target_file: src/inspector_model/editors/lines.rs
line_hint: 32
author: 
created: 
title: "summary"
tags: []
signature: "pub fn summary (text : & str) -> String"
---

How many lines there are — `(empty)`, `1 line`, `N lines`.

The three shapes and the count are the egui shell's
`crate::lines_editor::resumo` and `count_lines`. They are written again
here because `crawrad-ide` is a **binary** crate and this module belongs to
another `[[bin]]` — `lines_editor` is not in scope for `crawrad-makepad`.
The divergence is locked by a test.

# Examples

```rust
use crawrad_actions::inspector_model::editors::lines::summary;

assert_eq!(summary(""), "(empty)");
assert_eq!(summary("one"), "1 line");
assert_eq!(summary("one\ntwo"), "2 lines");
```
