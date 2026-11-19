---
id: nodeid
kind: doc
target_file: src/lib.rs
line_hint: 69
author: 
created: 
title: "NodeId"
tags: []
signature: "pub type NodeId = String"
---

The **stable** identity of a node.

Born with it, never changes, and it is to
this that `Value::Ref` points.

Not to be confused with `Name`: `Name` is the label that the user edits and
that code generation turns into a field. They were the same thing until
v2, and that confusion cost three mechanisms — rewiring referrers upon
renaming, hunting dangling references, and disconnecting upon deletion.

# Examples

```rust
use crawrad_ir::NodeId;

let id: NodeId = "btn_ok".to_string();
assert_eq!(id, "btn_ok");
```
