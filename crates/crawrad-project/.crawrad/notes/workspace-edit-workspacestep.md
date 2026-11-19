---
id: workspace-edit-workspacestep
kind: doc
target_file: src/workspace_edit.rs
line_hint: 186
author: 
created: 
title: "WorkspaceStep"
tags: []
signature: "pub enum WorkspaceStep"
---

Action the shell takes after a workspace command.

What the shell does after a workspace command: open the project created,
open the New Project dialog to add one, or say what happened.

# Examples

```rust
use std::path::PathBuf;
use crawrad_project::workspace_edit::WorkspaceStep;

let step = WorkspaceStep::Say("done".into());
assert_eq!(step, WorkspaceStep::Say("done".into()));
```
