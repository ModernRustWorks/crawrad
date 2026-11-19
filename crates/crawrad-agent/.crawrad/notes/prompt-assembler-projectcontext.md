---
id: prompt-assembler-projectcontext
kind: doc
target_file: src/prompt_assembler.rs
line_hint: 28
author: 
created: 
title: "ProjectContext"
tags: []
signature: "pub enum ProjectContext"
---

Live project context extracted from the active IDE state.

# Examples

```rust
use crawrad_agent::ProjectContext;

let ctx = ProjectContext::None;
assert_eq!(ctx, ProjectContext::None);
```
