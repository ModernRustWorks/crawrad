---
id: handler-name-is-derived-handler-name
kind: doc
target_file: src/handler_name.rs
line_hint: 27
author: 
created: 
title: "is_derived_handler_name"
tags: []
signature: "pub fn is_derived_handler_name (component : & str , event : & str , name : & str) -> bool"
---

Returns whether `name` matches a derived handler rule.

Returns true if `name` matches either the current or legacy derived handler naming rule for `component` and `event`.

# Examples

```rust
use crawrad_ir::is_derived_handler_name;

assert!(is_derived_handler_name("btn_ok", "on_click", "btn_ok_click"));
assert!(is_derived_handler_name("btn_ok", "on_click", "btn_okClick"));
assert!(!is_derived_handler_name("btn_ok", "on_click", "custom"));
```
