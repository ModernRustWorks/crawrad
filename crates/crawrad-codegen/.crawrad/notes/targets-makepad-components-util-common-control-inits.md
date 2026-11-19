---
id: targets-makepad-components-util-common-control-inits
kind: doc
target_file: src/targets/makepad/components/util.rs
line_hint: 27
author: 
created: 
title: "common_control_inits"
tags: []
signature: "pub fn common_control_inits (node : & Node) -> Vec < String >"
---

Returns the common control initializations (`visible: ...`, `enabled: ...`).

# Examples

```rust
use crawrad_codegen::targets::makepad::components::util::common_control_inits;
use crawrad_ir::Node;

let node = Node::new("btn", "Button");
let inits = common_control_inits(&node);
assert_eq!(inits.len(), 2);
```
