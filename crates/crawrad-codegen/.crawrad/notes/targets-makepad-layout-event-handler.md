---
id: targets-makepad-layout-event-handler
kind: doc
target_file: src/targets/makepad/layout.rs
line_hint: 33
author: 
created: 
title: "event_handler"
tags: []
signature: "pub fn event_handler (n : & Node , event : & str) -> Option < String >"
---

Retrieves an event handler name if defined.

# Examples

```rust
use crawrad_codegen::targets::makepad::layout::event_handler;
use crawrad_ir::Node;

let node = Node::new("btn", "Button");
assert_eq!(event_handler(&node, "on_click"), None);
```
