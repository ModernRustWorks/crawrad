---
id: targets-makepad-layout-text
kind: doc
target_file: src/targets/makepad/layout.rs
line_hint: 12
author: 
created: 
title: "text"
tags: []
signature: "pub fn text (n : & Node) -> String"
---

Returns the display text for a node (its text property or its name).

**Without the classic accelerator**, same as the egui twin
(`targets/egui/layout.rs:102`). A `&OK` in the designer is an `OK` with the
`O` underlined; a `&` that reaches the generated application is a literal
ampersand on a button, and the two targets were showing the same form
differently — `OK` on one side, `&OK` on the other. Measured on screen on
17/09/2026, in the very screenshot that proved the drawing.

# Examples

```rust
use crawrad_codegen::targets::makepad::layout::text;
use crawrad_ir::Node;

let node = Node::new("btn", "Button");
assert_eq!(text(&node), "btn");
```
