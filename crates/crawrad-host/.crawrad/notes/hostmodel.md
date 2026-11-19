---
id: hostmodel
kind: doc
target_file: src/lib.rs
line_hint: 109
author: 
created: 
title: "HostModel"
tags: []
signature: "pub struct HostModel"
---

Non-visual IDE state model (ADR 0044).

PR 1 introduced `doc` and `selection`.
PR 2 adds `center`, `active_path`, and `editor_buffers` so that read-only
MCP tools can be migrated to `&HostModel` parameters and stop holding a
reference to the heavyweight `IdeApp`.

# Examples

```rust
use crawrad_host::HostModel;

let host = HostModel::new();
assert!(!host.has_doc());
```
