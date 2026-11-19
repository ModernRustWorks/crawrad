---
id: props-props-message-dialog-buttons
kind: doc
target_file: src/props.rs
line_hint: 1947
author: 
created: 
title: "Props::message_dialog_buttons"
tags: []
signature: "pub fn message_dialog_buttons (default : & str) -> PropDesc"
---

Buttons layout for a message dialog (`Buttons`, #500).

# Examples

```rust
use crawrad_registry::Props;

let p = Props::message_dialog_buttons("mbYesNo");
assert_eq!(p.key, "buttons");
```
