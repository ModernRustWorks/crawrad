---
id: ide-app-lsp-service-ideapp-squiggles-json
kind: doc
target_file: src/ide_app/lsp_service.rs
line_hint: 707
author: 
created: 
title: "IdeApp::squiggles_json"
tags: []
signature: "pub fn squiggles_json (& self) -> serde_json :: Value"
---

The squiggles on the editor, as `editor.get_decorations` reads them.

# Examples

```rust,ignore
// binary crate: doctests cannot reach main.rs items
let squiggles = app.squiggles_json();
assert!(squiggles.is_array());
```
