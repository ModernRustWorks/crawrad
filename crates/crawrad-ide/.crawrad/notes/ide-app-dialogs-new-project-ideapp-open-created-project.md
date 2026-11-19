---
id: ide-app-dialogs-new-project-ideapp-open-created-project
kind: doc
target_file: src/ide_app/dialogs/new_project.rs
line_hint: 667
author: 
created: 
title: "IdeApp::open_created_project"
tags: []
signature: "pub fn open_created_project (& mut self , cx : & mut Cx , raiz : & std :: path :: Path)"
---

Opens the newly created project in the designer or editor.

Opens the form of the newly created project, when there is one, or opens the main code in the editor.

# Examples

```rust,ignore
// binary crate: doctests cannot reach main.rs items
let mut app = IdeApp::default();
assert!(app.raiz().is_none());
```
