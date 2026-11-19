---
id: permission-operationkind
kind: doc
target_file: src/permission.rs
line_hint: 15
author: 
created: 
title: "OperationKind"
tags: []
signature: "pub enum OperationKind"
---

Operation kinds, for classification and visual badges.

# Examples

```rust
use crawrad_agent::OperationKind;

assert_eq!(OperationKind::Read.badge(), "READ");
```
