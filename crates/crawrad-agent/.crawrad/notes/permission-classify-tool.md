---
id: permission-classify-tool
kind: doc
target_file: src/permission.rs
line_hint: 328
author: 
created: 
title: "classify_tool"
tags: []
signature: "pub fn classify_tool (tool_name : & str) -> OperationKind"
---

Classifies the tool from its name and operation pattern.

# Examples

```rust
use crawrad_agent::{OperationKind, classify_tool};

assert_eq!(classify_tool("fs.read_file"), OperationKind::Read);
assert_eq!(classify_tool("fs.write_file"), OperationKind::Write);
assert_eq!(classify_tool("bash.exec"), OperationKind::Execute);
```
