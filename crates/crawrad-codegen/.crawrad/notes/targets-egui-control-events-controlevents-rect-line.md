---
id: targets-egui-control-events-controlevents-rect-line
kind: doc
target_file: src/targets/egui/control_events.rs
line_hint: 189
author: 
created: 
title: "ControlEvents::rect_line"
tags: []
signature: "pub fn rect_line (& self , n : & Node , name : & str) -> String"
---

Emits the line recording a control rectangle for the router.

The line that leaves `n`'s rect for the router, right after `n` is
drawn (`r` is its rect there); empty for a control the router does not
serve. A hidden or disabled control leaves none, and so gets no event
(#431); the clip keeps a control scrolled out of sight from taking
the pointer.

# Examples

```rust
use crawrad_codegen::targets::egui::control_events::plan;
use crawrad_ir::Node;
use crawrad_registry::Registry;

let reg = Registry::builtin();
let mut node = Node::new("btn", "Button");
node.events.insert("on_mouse_down".to_string(), "btn_down".to_string());
let ce = plan(&reg, &[&node], |_| Ok(String::new())).unwrap().unwrap();
let line = ce.rect_line(&node, "btn");
assert!(line.contains("crawrad_rects.push"));
```
