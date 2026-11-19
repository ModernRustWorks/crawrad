---
id: ide-app-status-bar-ideapp-project-kind-now
kind: doc
target_file: src/ide_app/status_bar.rs
line_hint: 118
author: 
created: 
title: "IdeApp::project_kind_now"
tags: []
signature: "pub (super) fn project_kind_now (& self) -> (Option < & 'static str >, bool)"
---

What the status bar says and whether the open project has no form.

Returns `(kind, without_form)` where `without_form` is `p.open && !p.has_forms`.
The kind is `console` or `library` when there is no UI framework yet. It reads
the disk, so it is measured only when the root or the active file changed.

# Examples

```rust
// Reads the project context once to get both the kind and without_form flag.
let (kind, without_form) = app.project_kind_now();
```
