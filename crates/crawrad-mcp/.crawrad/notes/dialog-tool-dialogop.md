---
id: dialog-tool-dialogop
kind: doc
target_file: src/dialog_tool.rs
line_hint: 13
author: 
created: 
title: "DialogOp"
tags: []
signature: "pub enum DialogOp"
---

An operation to execute on a control of the open dialog.

# Examples

```
use crawrad_mcp::dialog_tool::DialogOp;

let op = DialogOp::Press {
    control: "btn_ok".to_string(),
};
assert_eq!(op.control(), "btn_ok");
assert_eq!(op.kind_str(), "press");
```
