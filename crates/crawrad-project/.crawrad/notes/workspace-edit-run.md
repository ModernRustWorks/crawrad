---
id: workspace-edit-run
kind: doc
target_file: src/workspace_edit.rs
line_hint: 194
author: 
created: 
title: "run"
tags: []
signature: "pub fn run (root : Option < & Path > , command : crawrad_actions :: FileEffect , pick : impl FnOnce () -> Option < PathBuf > ,) -> WorkspaceStep"
---

Runs a workspace modification command.

Runs Add Project / Add Existing Project / Set as Active Project (#471) for
the workspace at `root` (the open one). `pick` asks for a folder when the
command came without one (the menu); an agent passes it.

# Examples

```rust
use crawrad_actions::FileEffect;
use crawrad_project::workspace_edit::{run, WorkspaceStep};

let step = run(None, FileEffect::NewFile, || None);
assert!(matches!(step, WorkspaceStep::Say(_)));
```
