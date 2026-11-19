---
id: events-method-params
kind: doc
target_file: src/events.rs
line_hint: 88
author: 
created: 
title: "method_params"
tags: []
signature: "pub fn method_params (reg : & crawrad_registry :: Registry , class : & str , event : & str) -> String"
---

Returns the parameter list for an event handler.

The parameter list for the handler of `event`, as `pub fn name(...)` wants
it — taken from the signature the catalogue declares, never guessed.

`class` only decides **where to look first**: an event name is unique
across the catalogue in practice, and the sweep afterwards is what makes a
handler on a class that does not declare the event still come out with the
declared parameters instead of a bare `&mut self`.

# Examples

```rust
use crawrad_codegen::events::method_params;
use crawrad_registry::Registry;

let reg = Registry::builtin();
let params = method_params(&reg, "Button", "on_click");
assert_eq!(params, "&mut self");
```
