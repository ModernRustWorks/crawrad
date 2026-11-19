---
id: workspace-workspace-manifest
kind: doc
target_file: src/workspace.rs
line_hint: 69
author: 
created: 
title: "workspace_manifest"
tags: []
signature: "pub fn workspace_manifest (first_member : & str) -> String"
---

The manifest text that New Project writes at the workspace root.

The manifest a New Project writes at the workspace root (#471): a virtual
manifest whose only member is the first project, which is also the active
one (`default-members`, read back by [`members`]).

# Examples

```rust
use crawrad_project::workspace::workspace_manifest;

let toml = workspace_manifest("my_app");
assert!(toml.contains("my_app"));
```
