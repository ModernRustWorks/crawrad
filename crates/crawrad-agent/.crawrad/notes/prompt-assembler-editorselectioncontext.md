---
id: prompt-assembler-editorselectioncontext
kind: doc
target_file: src/prompt_assembler.rs
line_hint: 17
author: 
created: 
title: "EditorSelectionContext"
tags: []
signature: "pub struct EditorSelectionContext"
---

Selection range and text inside the code editor.

# Examples

```rust
use crawrad_agent::EditorSelectionContext;

let sel = EditorSelectionContext {
    start_line: 1,
    end_line: 1,
    selected_text: "code".to_string(),
};
assert_eq!(sel.start_line, 1);
```
