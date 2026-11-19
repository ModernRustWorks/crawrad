---
id: designer-tool-wired-2
kind: doc
target_file: src/designer_tool.rs
line_hint: 324
author: 
created: 
title: "wired"
tags: []
signature: "pub fn wired (w : & Wired , rev : u64 , file : Option < String > , line : Option < usize > , generated : bool , note : Option < String > ,) -> Answer"
---

`designer.wire_event`, the answer once the stub was looked for.

# Examples

```rust
use crawrad_mcp::designer_tool::{wired, Wired};

let w = Wired {
    node: "btn1".into(),
    name: "btn1".into(),
    event: "OnClick".into(),
    handler: "btn1_click".into(),
};
let (res, outcome) = wired(&w, 1, None, None, false, None);
assert_eq!(outcome, "applied");
```
