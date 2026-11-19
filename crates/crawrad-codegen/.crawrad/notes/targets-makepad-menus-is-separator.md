---
id: targets-makepad-menus-is-separator
kind: doc
target_file: src/targets/makepad/menus.rs
line_hint: 111
author: 
created: 
title: "is_separator"
tags: []
signature: "pub fn is_separator (item : & Node) -> bool"
---

Whether the item is a separator (a `-` caption, as in the classic RAD IDE).

# Examples

```rust
use crawrad_codegen::targets::makepad::menus::is_separator;
use crawrad_ir::Node;

let mut item = Node::new("item1", "MenuItem");
item.props.insert("text".to_string(), crawrad_ir::Value::Str("-".to_string()));
assert!(is_separator(&item));
```
