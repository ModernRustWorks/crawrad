---
id: ide-app-chat-ideapp-extrair-selecao-do-editor
kind: doc
target_file: src/ide_app/chat.rs
line_hint: 493
author: 
created: 
title: "IdeApp::extrair_selecao_do_editor"
tags: []
signature: "pub fn extrair_selecao_do_editor (& self) -> Option < crawrad_agent :: EditorSelectionContext >"
---

Extracts the current text selection from the active CodeView (if any).

# Examples

```rust,ignore
// binary crate: doctests cannot reach main.rs items
let app = IdeApp::default();
assert_eq!(app.extrair_selecao_do_editor(), None);
```
