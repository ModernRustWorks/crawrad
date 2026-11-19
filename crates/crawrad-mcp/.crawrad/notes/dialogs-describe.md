---
id: dialogs-describe
kind: doc
target_file: src/dialogs.rs
line_hint: 114
author: 
created: 
title: "describe"
tags: []
signature: "pub fn describe (spec : & DialogSpec , rows : & [(String , [f64 ; 4])] , read : impl Fn (& Control) -> ControlState ,) -> serde_json :: Value"
---

`view.get_dialog`: the open dialog's name and every control as drawn.

# Examples

```rust
use crawrad_mcp::dialogs::{describe, ControlState, NEW_PROJECT};

let val = describe(&NEW_PROJECT, &[], |_| ControlState::default());
assert_eq!(val["open"], "new_project");
```
