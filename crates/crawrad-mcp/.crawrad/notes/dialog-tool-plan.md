---
id: dialog-tool-plan
kind: doc
target_file: src/dialog_tool.rs
line_hint: 57
author: 
created: 
title: "plan"
tags: []
signature: "pub fn plan (name : & str , args : & Value , open : Option < & DialogSpec > ,) -> Option < Result < DialogOp , Answer > >"
---

Plans an operation on the currently open dialog.

# Examples

```
use crawrad_mcp::dialogs::OPTIONS;
use crawrad_mcp::dialog_tool::{DialogOp, plan};
use serde_json::json;

let args = json!({ "control": "opt_close" });
let planned = plan("dialog.press", &args, Some(&OPTIONS));
assert_eq!(
    planned.and_then(Result::ok),
    Some(DialogOp::Press { control: "opt_close".into() })
);
```

# Errors

Returns an error answer when no dialog is open, arguments are invalid,
the control is unknown, or the control kind does not support the operation.
