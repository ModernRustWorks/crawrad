---
id: code-lens-at-line
kind: doc
target_file: src/code_lens.rs
line_hint: 131
author: 
created: 
title: "at_line"
tags: []
signature: "pub fn at_line (lenses : & [CodeLens] , line : usize) -> Option < & LensCommand >"
---

Finds the command triggered by clicking on a given line.

The command a click on line runs: the first `ShowLocations`, else the first command.

# Examples

```rust
use crawrad_lsp::code_lens::at_line;

let cmd = at_line(&[], 0);
assert!(cmd.is_none());
```
