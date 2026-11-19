---
id: dialog-tool-dialogop-control
kind: doc
target_file: src/dialog_tool.rs
line_hint: 38
author: 
created: 
title: "DialogOp::control"
tags: []
signature: "pub fn control (& self) -> & str"
---

Returns the target control name.

# Examples

```
use crawrad_mcp::dialog_tool::DialogOp;

let op = DialogOp::Press {
    control: "opt_close".into(),
};
assert_eq!(op.control(), "opt_close");
```
