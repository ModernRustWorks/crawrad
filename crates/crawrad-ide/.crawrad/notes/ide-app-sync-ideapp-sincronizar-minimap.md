---
id: ide-app-sync-ideapp-sincronizar-minimap
kind: doc
target_file: src/ide_app/sync.rs
line_hint: 147
author: 
created: 
title: "IdeApp::sincronizar_minimap"
tags: []
signature: "pub fn sincronizar_minimap (& mut self , cx : & mut Cx)"
---

Synchronizes the minimap with the content and scroll position of the CodeView.

# Examples

```rust,ignore
// binary crate: doctests cannot reach main.rs items
app.sincronizar_minimap(cx);
assert_eq!(app.minimap_visivel, true);
```
