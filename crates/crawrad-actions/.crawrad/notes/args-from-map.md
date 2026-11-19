---
id: args-from-map
kind: doc
target_file: src/lib.rs
line_hint: 232
author: 
created: 
title: "Args::from_map"
tags: []
signature: "pub fn from_map (values : BTreeMap < String , Value >) -> Self"
---

Builds from a map of names and values.

# Examples

```rust
use crawrad_actions::Args;
use crawrad_ir::Value;
use std::collections::BTreeMap;

let mut map = BTreeMap::new();
map.insert("key".into(), Value::Int(42));
let args = Args::from_map(map);
assert_eq!(args.as_number("key"), Some(42.0));
```
