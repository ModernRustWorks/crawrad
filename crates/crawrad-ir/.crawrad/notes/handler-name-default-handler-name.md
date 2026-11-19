---
id: handler-name-default-handler-name
kind: doc
target_file: src/handler_name.rs
line_hint: 4
author: 
created: 
title: "default_handler_name"
tags: []
signature: "pub fn default_handler_name (component : & str , event : & str) -> String"
---

Default handler name for a component and event.

Default handler name for a component and event: `btn_ok` + `on_click` = `btn_ok_click`.

# Examples

```rust
use crawrad_ir::default_handler_name;

assert_eq!(default_handler_name("btn_ok", "on_click"), "btn_ok_click");
```
