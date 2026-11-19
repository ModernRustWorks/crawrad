---
id: permission-permissionmanager-classify-tool
kind: doc
target_file: src/permission.rs
line_hint: 406
author: 
created: 
title: "PermissionManager::classify_tool"
tags: []
signature: "pub fn classify_tool (tool_name : & str) -> OperationKind"
---

Classifies a tool by its name.

# Examples

```rust
use crawrad_agent::{OperationKind, PermissionManager};

assert_eq!(PermissionManager::classify_tool("read"), OperationKind::Read);
```
