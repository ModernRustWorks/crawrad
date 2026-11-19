---
id: ide-app-view-state-ideapp-clear-selection
kind: doc
target_file: src/ide_app/view_state.rs
line_hint: 80
author: 
created: 
title: "IdeApp::clear_selection"
tags: []
signature: "pub fn clear_selection (& mut self , cx : & mut Cx)"
---

Clears the active selection.

# Examples

```rust,ignore
// binary crate: doctests cannot reach main.rs items
app.clear_selection(cx);
assert_eq!(app.no_ativo(), "—");
```
