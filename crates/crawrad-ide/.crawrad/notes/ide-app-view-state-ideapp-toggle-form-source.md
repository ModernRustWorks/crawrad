---
id: ide-app-view-state-ideapp-toggle-form-source
kind: doc
target_file: src/ide_app/view_state.rs
line_hint: 176
author: 
created: 
title: "IdeApp::toggle_form_source"
tags: []
signature: "pub fn toggle_form_source (& mut self , cx : & mut Cx)"
---

`Alt+F12`: form source in place of design.

# Examples

```rust,ignore
// binary crate: doctests cannot reach main.rs items
let prev = app.host.center();
app.toggle_form_source(cx);
assert_ne!(app.host.center(), prev);
```
