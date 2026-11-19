---
id: props-message-dialog-buttons
kind: doc
target_file: src/props.rs
line_hint: 79
author: 
created: 
title: "MESSAGE_DIALOG_BUTTONS"
tags: []
signature: "pub const MESSAGE_DIALOG_BUTTONS : & [& str] = & [\"mbOK\" , \"mbOKCancel\" , \"mbYesNo\" , \"mbYesNoCancel\"]"
---

The button layouts of a `MessageDialog` (#500).

# Examples

```rust
use crawrad_registry::MESSAGE_DIALOG_BUTTONS;

assert!(MESSAGE_DIALOG_BUTTONS.contains(&"mbOK"));
```
