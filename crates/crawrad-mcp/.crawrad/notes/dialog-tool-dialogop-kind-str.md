---
id: dialog-tool-dialogop-kind-str
kind: doc
target_file: src/dialog_tool.rs
line_hint: 47
author: 
created: 
title: "DialogOp::kind_str"
tags: []
signature: "pub fn kind_str (& self) -> & 'static str"
---

Returns a short string identifying the operation kind.

# Examples

```
use crawrad_mcp::dialog_tool::DialogOp;

let op = DialogOp::Press {
    control: "btn".into(),
};
assert_eq!(op.kind_str(), "press");
```
