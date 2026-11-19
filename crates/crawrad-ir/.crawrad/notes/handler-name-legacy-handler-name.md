---
id: handler-name-legacy-handler-name
kind: doc
target_file: src/handler_name.rs
line_hint: 10
author: 
created: 
title: "legacy_handler_name"
tags: []
signature: "pub fn legacy_handler_name (component : & str , event : & str) -> String"
---

Legacy handler name rule: `btn_ok` + `on_click` = `btn_okClick`.

# Examples

```rust
use crawrad_ir::legacy_handler_name;

assert_eq!(legacy_handler_name("btn_ok", "on_click"), "btn_okClick");
```
