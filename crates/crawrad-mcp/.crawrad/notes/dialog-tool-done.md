---
id: dialog-tool-done
kind: doc
target_file: src/dialog_tool.rs
line_hint: 164
author: 
created: 
title: "done"
tags: []
signature: "pub fn done (op : & DialogOp , dialog : & str , driven : Result < () , & str >) -> Answer"
---

Builds the answer for an executed dialog operation.

`driven` is what the widget drive answered: `Err(code)` becomes that
refusal — `hidden-control` when the control is not on screen, `unknown-item`
when a `Pick`'s text is not an item of the drop-down.

# Examples

```
use crawrad_mcp::dialog_tool::{DialogOp, done};

let op = DialogOp::Press {
    control: "btn_ok".to_string(),
};
let (result, outcome) = done(&op, "options", Ok(()));
assert_eq!(outcome, "applied");
assert!(!result.is_error);
```
