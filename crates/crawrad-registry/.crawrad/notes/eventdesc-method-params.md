---
id: eventdesc-method-params
kind: doc
target_file: src/lib.rs
line_hint: 255
author: 
created: 
title: "EventDesc::method_params"
tags: []
signature: "pub fn method_params (& self) -> String"
---

Returns the parameter list for handler method declaration.

Returns the parameter list for handler method declaration (`pub fn {f}(...)`),
prefixing additional parameters with `_` to avoid warnings when the generated body
is still empty.

# Examples

```rust
use crawrad_registry::EventDesc;

let ev = EventDesc::new("on_click", "OnClick", "fn(&mut self)");
assert_eq!(ev.method_params(), "&mut self");
```
