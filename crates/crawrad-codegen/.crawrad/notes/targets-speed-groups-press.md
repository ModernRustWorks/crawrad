---
id: targets-speed-groups-press
kind: doc
target_file: src/targets/speed_groups.rs
line_hint: 23
author: 
created: 
title: "press"
tags: []
signature: "pub fn press (doc : & Document , node : & Node , me : & str , prefix : & str) -> String"
---

Emits speed group toggle code executed on button press.

What a click does to the group, run when `me`'s `group_index` is above
0: `prefix` is the way the generated code reaches the form (`self.` on
egui, `self.form.` on Makepad).

# Examples

```rust
use crawrad_codegen::targets::speed_groups::press;
use crawrad_ir::{Document, Node};

let doc = Document::new("form", "Form");
let node = Node::new("btn", "SpeedButton");
let code = press(&doc, &node, "btn", "self.");
assert!(code.contains("self.btn.down"));
```
