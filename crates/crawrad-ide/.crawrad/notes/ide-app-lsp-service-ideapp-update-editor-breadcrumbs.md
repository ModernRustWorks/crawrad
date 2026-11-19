---
id: ide-app-lsp-service-ideapp-update-editor-breadcrumbs
kind: doc
target_file: src/ide_app/lsp_service.rs
line_hint: 776
author: 
created: 
title: "IdeApp::update_editor_breadcrumbs"
tags: []
signature: "pub fn update_editor_breadcrumbs (& mut self , cx : & mut Cx , _arquivo : & Path)"
---

Rewrites the editor's breadcrumb for `_arquivo` (#604), in the footer.

# Examples

```rust,ignore
// binary crate: doctests cannot reach main.rs items
app.update_editor_breadcrumbs(cx, Path::new("src/main.rs"));
assert_eq!(app.inlay_hints_off, false);
```
