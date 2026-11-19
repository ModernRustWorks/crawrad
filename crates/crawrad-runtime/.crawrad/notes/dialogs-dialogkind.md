---
id: dialogs-dialogkind
kind: doc
target_file: src/dialogs.rs
line_hint: 15
author: 
created: 
title: "DialogKind"
tags: []
signature: "pub enum DialogKind"
---

Visual classification and icon for the message dialog.

# Examples

```rust
use crawrad_runtime::dialogs::DialogKind;

let kind = DialogKind::Information;
assert_eq!(kind.default_title(), "Information");
```
