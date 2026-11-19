---
id: props-props-message-dialog-kind
kind: doc
target_file: src/props.rs
line_hint: 1935
author: 
created: 
title: "Props::message_dialog_kind"
tags: []
signature: "pub fn message_dialog_kind (default : & str) -> PropDesc"
---

Dialog kind / icon classification (`Kind`, #500).

# Examples

```rust
use crawrad_registry::Props;

let p = Props::message_dialog_kind("mtConfirmation");
assert_eq!(p.key, "kind");
```
