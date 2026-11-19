---
id: adapter-portadb-bridge-value-to-text
kind: doc
target_file: src/adapter/portadb_bridge.rs
line_hint: 97
author: 
created: 
title: "value_to_text"
tags: []
signature: "pub fn value_to_text (val : & Value) -> String"
---

Formats a [`Value`] into a string for visual bindings.

Formats a [`Value`] into its canonical string representation for visual bindings.

# Examples

```rust
use portadb::Value;
use crawrad_binding::adapter::portadb_bridge::value_to_text;

assert_eq!(value_to_text(&Value::Integer(42)), "42");
```
