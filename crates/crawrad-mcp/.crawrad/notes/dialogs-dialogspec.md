---
id: dialogs-dialogspec
kind: doc
target_file: src/dialogs.rs
line_hint: 76
author: 
created: 
title: "DialogSpec"
tags: []
signature: "pub struct DialogSpec"
---

A dialog: the name `view.get_dialog` reports, and its controls.

# Examples

```rust
use crawrad_mcp::dialogs::DialogSpec;

let spec = DialogSpec {
    name: "test",
    root: ["root", "dlg"],
    controls: &[],
};
assert_eq!(spec.name, "test");
```
