---
id: ide-app-ideapp-no-form-in-project
kind: doc
target_file: src/ide_app.rs
line_hint: 1083
author: 
created: 
title: "IdeApp::no_form_in_project"
tags: []
signature: "pub (crate) fn no_form_in_project (& self) -> bool"
---

Whether the active open project has no form and no open document.

When true, side panels show empty states and the rail omits the inspector.

# Examples

```rust
// Returns true when the project is open without forms and doc is None.
let no_form = app.no_form_in_project();
```
