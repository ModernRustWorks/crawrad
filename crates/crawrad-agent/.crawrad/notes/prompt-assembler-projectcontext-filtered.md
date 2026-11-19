---
id: prompt-assembler-projectcontext-filtered
kind: doc
target_file: src/prompt_assembler.rs
line_hint: 51
author: 
created: 
title: "ProjectContext::filtered"
tags: []
signature: "pub fn filtered (self , form : bool , selection : bool) -> Self"
---

Filters the project context according to form and selection flags.

The context the chat's `Form` / `Sel` switches let through (#420): with
`form` off nothing of the open file or form goes; with `selection` off
the file or form goes without what is selected in it.

# Examples

```rust
use crawrad_agent::ProjectContext;

let ctx = ProjectContext::None.filtered(false, false);
assert_eq!(ctx, ProjectContext::None);
```
