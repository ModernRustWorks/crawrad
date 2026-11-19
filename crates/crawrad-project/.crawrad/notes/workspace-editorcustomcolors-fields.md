---
id: workspace-editorcustomcolors-fields
kind: doc
target_file: src/workspace.rs
line_hint: 716
author: 
created: 
title: "EditorCustomColors::fields"
tags: []
signature: "pub fn fields (& self) -> [& str ; 9]"
---

Returns the nine colors in the order the Options page lists their inputs.

The nine colors in the order the Options page lists their inputs:
keyword, type, function, identifier, number, string, comment,
delimiter, background.

# Examples

```rust
use crawrad_project::workspace::EditorCustomColors;

let c = EditorCustomColors::default();
assert_eq!(c.fields().len(), 9);
```
