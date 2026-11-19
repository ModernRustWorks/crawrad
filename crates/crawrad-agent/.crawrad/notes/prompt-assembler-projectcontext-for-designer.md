---
id: prompt-assembler-projectcontext-for-designer
kind: doc
target_file: src/prompt_assembler.rs
line_hint: 115
author: 
created: 
title: "ProjectContext::for_designer"
tags: []
signature: "pub fn for_designer (form_name : String , form_path : Option < & Path > , root : Option < & Path > , component_count : usize , selected_components : Vec < String > ,) -> Self"
---

Builds a Designer context from form details.

# Examples

```rust
use crawrad_agent::ProjectContext;

let ctx = ProjectContext::for_designer("MainForm".into(), None, None, 0, vec![]);
assert!(matches!(ctx, ProjectContext::Designer { .. }));
```
