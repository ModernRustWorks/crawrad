---
id: targets-speed-groups-siblings
kind: doc
target_file: src/targets/speed_groups.rs
line_hint: 13
author: 
created: 
title: "siblings"
tags: []
signature: "pub fn siblings (doc : & Document , node : & Node) -> Vec < String >"
---

The fields of the other SpeedButtons under `node`'s parent.

# Examples

```rust
use crawrad_codegen::targets::speed_groups::siblings;
use crawrad_ir::{Document, Node};

let doc = Document::new("form", "Form");
let node = Node::new("btn", "SpeedButton");
let sibs = siblings(&doc, &node);
assert!(sibs.is_empty());
```
