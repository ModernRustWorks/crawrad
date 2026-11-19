---
id: char-case-cased-for
kind: doc
target_file: src/char_case.rs
line_hint: 24
author: 
created: 
title: "cased_for"
tags: []
signature: "pub fn cased_for (node : & Node , text : & str) -> String"
---

`text` in `node`'s own `CharCase`; a node without one leaves it.

# Examples

```rust
use crawrad_ir::char_case::cased_for;
use crawrad_ir::{Node, Value};

let mut node = Node::new("edt", "Edit");
assert_eq!(cased_for(&node, "Hello"), "Hello");
node.props.insert("char_case".into(), Value::Enum("ecUpperCase".into()));
assert_eq!(cased_for(&node, "Hello"), "HELLO");
```
