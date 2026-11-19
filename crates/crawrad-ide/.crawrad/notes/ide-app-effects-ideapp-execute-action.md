---
id: ide-app-effects-ideapp-execute-action
kind: doc
target_file: src/ide_app/effects.rs
line_hint: 8
author: 
created: 
title: "IdeApp::execute_action"
tags: []
signature: "pub fn execute_action (& mut self , cx : & mut Cx , action_id : & str)"
---

Executes a registered action by its unique action ID via `crawrad-actions`.

# Examples

```rust,ignore
// binary crate: doctests cannot reach main.rs items
let mut app = IdeApp::default();
assert!(app.status.is_none());
```
