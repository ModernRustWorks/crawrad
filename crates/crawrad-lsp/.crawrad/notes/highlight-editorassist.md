---
id: highlight-editorassist
kind: doc
target_file: src/highlight.rs
line_hint: 204
author: 
created: 
title: "EditorAssist"
tags: []
signature: "pub struct EditorAssist"
---

Aggregates language server worker threads supporting editor features.

The editor's helpers that ask rust-analyzer on their own threads: the
grey types inside the lines, the occurrences of the name under the
caret and the `Ctrl+Space` completions. One value, so the IDE keeps one field for both.

# Examples

```rust
use crawrad_lsp::highlight::EditorAssist;

let assist = EditorAssist::default();
assert!(assist.hints.take().is_none());
```
