---
id: code-action-codeaction
kind: doc
target_file: src/code_action.rs
line_hint: 34
author: 
created: 
title: "CodeAction"
tags: []
signature: "pub struct CodeAction"
---

One action the IDE can apply along with its title, kind, and edits.

One action the IDE can apply: its title, its kind (`quickfix`,
`refactor.rewrite`…) and its edit.

# Examples

```rust
use crawrad_lsp::code_action::CodeAction;
use crawrad_lsp::edit::WorkspaceEdit;

let action = CodeAction {
    title: "Import HashMap".to_string(),
    kind: Some("quickfix".to_string()),
    preferred: true,
    edit: WorkspaceEdit::default(),
};
assert_eq!(action.title, "Import HashMap");
```
