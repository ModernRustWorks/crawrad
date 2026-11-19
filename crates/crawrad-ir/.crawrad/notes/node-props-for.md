---
id: node-props-for
kind: doc
target_file: src/lib.rs
line_hint: 152
author: 
created: 
title: "Node::props_for"
tags: []
signature: "pub fn props_for < 'a > (& 'a self , target : & 'a str) -> impl Iterator < Item = (PropKey , & 'a Value) >"
---

Only the properties that the specified runtime should apply.

# Examples

```rust
use crawrad_ir::{Node, Value};

let mut node = Node::new("btn", "Button");
node.props.insert("text".to_string(), Value::Str("OK".to_string()));
let props: Vec<_> = node.props_for("egui").collect();
assert_eq!(props.len(), 1);
```
