---
id: events-handler
kind: doc
target_file: src/events.rs
line_hint: 103
author: 
created: 
title: "Handler"
tags: []
signature: "pub struct Handler < 'a >"
---

Represents an event handler wired in the user file.

One handler the user's file carries: the component that wired it, the
event it answers, and the name of the `fn`.

# Examples

```rust
use crawrad_codegen::events::Handler;
use crawrad_ir::Node;

let node = Node::new("btn", "Button");
let handler = Handler {
    node: &node,
    event: "on_click",
    name: "btn_click".to_string(),
};
assert_eq!(handler.event, "on_click");
```
