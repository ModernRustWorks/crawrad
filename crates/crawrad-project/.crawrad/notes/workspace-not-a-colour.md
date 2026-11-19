---
id: workspace-not-a-colour
kind: doc
target_file: src/workspace.rs
line_hint: 925
author: 
created: 
title: "NOT_A_COLOUR"
tags: []
signature: "pub const NOT_A_COLOUR : & str = \"Not a colour: use #RRGGBB\""
---

Error message for invalid color inputs.

# Examples

```rust
use crawrad_project::workspace::NOT_A_COLOUR;

assert_eq!(NOT_A_COLOUR, "Not a colour: use #RRGGBB");
```
