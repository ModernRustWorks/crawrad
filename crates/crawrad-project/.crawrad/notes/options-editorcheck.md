---
id: options-editorcheck
kind: doc
target_file: src/options.rs
line_hint: 287
author: 
created: 
title: "EditorCheck"
tags: []
signature: "pub enum EditorCheck"
---

A check box of how the editor looks.

A check box of how the editor looks — on the Editor page, Commonly Used
and the search results: what it says, the preference it reads and
writes, and every widget that shows it.

# Examples

```rust
use crawrad_project::options::EditorCheck;

assert_eq!(EditorCheck::ALL.len(), 4);
```
