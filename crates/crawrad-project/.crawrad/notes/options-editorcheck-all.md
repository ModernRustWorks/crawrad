---
id: options-editorcheck-all
kind: doc
target_file: src/options.rs
line_hint: 297
author: 
created: 
title: "EditorCheck::ALL"
tags: []
signature: "pub const ALL : [Self ; 4] = [Self :: LineNumbers , Self :: InlayHints , Self :: CurrentLine , Self :: BreadcrumbChevron ,]"
---

The four, in the order the Editor page shows them.

# Examples

```rust
use crawrad_project::options::EditorCheck;

assert_eq!(EditorCheck::ALL[0], EditorCheck::LineNumbers);
```
