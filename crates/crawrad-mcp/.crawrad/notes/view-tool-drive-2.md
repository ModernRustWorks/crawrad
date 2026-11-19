---
id: view-tool-drive-2
kind: doc
target_file: src/view_tool.rs
line_hint: 24
author: 
created: 
title: "drive"
tags: []
signature: "pub fn drive (name : & str , args : & Value , doc : Option < & crawrad_doc :: Doc > , known : impl Fn (& str) -> bool ,) -> Option < Result < Drive , Answer > >"
---

Parses arguments for drive tools into a Drive operation.

The drive tools' arguments, or `None` when `name` is not one of them.
`known` says whether an action id exists in the IDE.

# Examples

```rust
use crawrad_mcp::view_tool::drive;
use serde_json::json;

let args = json!({ "name": "Explorer" });
let op = drive("view.show_panel", &args, None, |_| false);
assert!(op.is_some());
```
