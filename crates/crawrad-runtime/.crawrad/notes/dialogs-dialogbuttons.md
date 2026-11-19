---
id: dialogs-dialogbuttons
kind: doc
target_file: src/dialogs.rs
line_hint: 88
author: 
created: 
title: "DialogButtons"
tags: []
signature: "pub enum DialogButtons"
---

Button layout configuration for the message dialog.

# Examples

```rust
use crawrad_runtime::dialogs::DialogButtons;

let buttons = DialogButtons::YesNo;
assert_eq!(buttons.as_name(), "mbYesNo");
```
