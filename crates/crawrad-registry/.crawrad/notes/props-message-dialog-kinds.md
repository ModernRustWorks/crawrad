---
id: props-message-dialog-kinds
kind: doc
target_file: src/props.rs
line_hint: 75
author: 
created: 
title: "MESSAGE_DIALOG_KINDS"
tags: []
signature: "pub const MESSAGE_DIALOG_KINDS : & [& str] = & [\"mtInformation\" , \"mtWarning\" , \"mtError\" , \"mtConfirmation\"]"
---

The kinds/icons of a `MessageDialog` (#500).

# Examples

```rust
use crawrad_registry::MESSAGE_DIALOG_KINDS;

assert!(MESSAGE_DIALOG_KINDS.contains(&"mtInformation"));
```
