---
id: hostmodel-project-forms
kind: doc
target_file: src/lib.rs
line_hint: 465
author: 
created: 
title: "HostModel::project_forms"
tags: []
signature: "pub fn project_forms (& self) -> Vec < (String , crawrad_ir :: Document) >"
---

Returns all project forms from open documents and disk files.

Returns the workspace root by walking up from `active_path` until a
`Cargo.toml` is found, or `None` if no file is active.

This is the same heuristic that `IdeApp::raiz_projeto` uses, centralised
here so MCP tools can call `host.workspace_root()` directly.
Every form of the project: the open one as it is in memory, the
others from the disk — a handler bound in any of them is the form's.

# Examples

```rust
use crawrad_host::HostModel;

let host = HostModel::default();
assert!(host.project_forms().is_empty());
```
