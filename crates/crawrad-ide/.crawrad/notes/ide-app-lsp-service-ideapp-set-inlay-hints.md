---
id: ide-app-lsp-service-ideapp-set-inlay-hints
kind: doc
target_file: src/ide_app/lsp_service.rs
line_hint: 629
author: 
created: 
title: "IdeApp::set_inlay_hints"
tags: []
signature: "pub fn set_inlay_hints (& mut self , cx : & mut Cx , on : bool)"
---

Enables or disables inlay hints and persists the preference.

Turns the inlay hints on or off — the catalog action and the Options
check box both land here — and remembers it in the preferences.

# Examples

```rust,ignore
// binary crate: doctests cannot reach main.rs items
app.set_inlay_hints(cx, true);
assert_eq!(app.inlay_hints_off, false);
```
