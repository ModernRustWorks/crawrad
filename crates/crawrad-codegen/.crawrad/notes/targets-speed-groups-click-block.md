---
id: targets-speed-groups-click-block
kind: doc
target_file: src/targets/speed_groups.rs
line_hint: 45
author: 
created: 
title: "click_block"
tags: []
signature: "pub fn click_block (doc : & Document , node : & Node , me : & str , prefix : & str , clicked : & str , calls : & str ,) -> String"
---

Generates complete click handling block for grouped speed buttons.

The whole click block: `clicked` is the click's condition, `calls` what
runs after the group moved (OnClick, the linked action). With no calls
the group test joins the click's condition, so no lone `if` is left
nested (`clippy::collapsible_if` in the user's project).

# Examples

```rust
use crawrad_codegen::targets::speed_groups::click_block;
use crawrad_ir::{Document, Node};

let doc = Document::new("form", "Form");
let node = Node::new("btn", "SpeedButton");
let code = click_block(&doc, &node, "btn", "self.", "resp.clicked()", "");
assert!(code.contains("resp.clicked()"));
```
