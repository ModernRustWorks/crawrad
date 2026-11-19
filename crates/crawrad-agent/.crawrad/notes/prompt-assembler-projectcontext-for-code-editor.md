---
id: prompt-assembler-projectcontext-for-code-editor
kind: doc
target_file: src/prompt_assembler.rs
line_hint: 94
author: 
created: 
title: "ProjectContext::for_code_editor"
tags: []
signature: "pub fn for_code_editor (path : & Path , root : Option < & Path > , live_content : String , selection : Option < EditorSelectionContext > ,) -> Self"
---

Builds a CodeEditor context from file path and content.

Builds a `CodeEditor` context from file path, relative root, live content, and optional selection.

# Examples

```rust
use crawrad_agent::ProjectContext;
use std::path::Path;

let ctx = ProjectContext::for_code_editor(Path::new("lib.rs"), None, "code".into(), None);
assert!(matches!(ctx, ProjectContext::CodeEditor { .. }));
```
