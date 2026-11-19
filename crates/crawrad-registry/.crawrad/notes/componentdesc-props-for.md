---
id: componentdesc-props-for
kind: doc
target_file: src/lib.rs
line_hint: 651
author: 
created: 
title: "ComponentDesc::props_for"
tags: []
signature: "pub fn props_for < 'a > (& 'a self , target : & 'a str) -> impl Iterator < Item = & 'a PropDesc >"
---

Only the properties that make sense in the given runtime.

# Examples

```rust
use crawrad_registry::button;

let desc = button();
let win32_props: Vec<_> = desc.props_for("win32").collect();
assert!(!win32_props.is_empty());
```
