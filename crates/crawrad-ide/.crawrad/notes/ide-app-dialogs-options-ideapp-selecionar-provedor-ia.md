---
id: ide-app-dialogs-options-ideapp-selecionar-provedor-ia
kind: doc
target_file: src/ide_app/dialogs/options.rs
line_hint: 115
author: 
created: 
title: "IdeApp::selecionar_provedor_ia"
tags: []
signature: "pub fn selecionar_provedor_ia (& mut self , cx : & mut Cx , provider_name : & str)"
---

Synchronizes the AI provider selection.

Synchronizes the AI provider selection from any combo box (Options ACP, Quick Options, or Search Results).

# Examples

```rust,ignore
// binary crate: doctests cannot reach main.rs items
let mut app = IdeApp::default();
assert!(!app.chat.busy);
```
