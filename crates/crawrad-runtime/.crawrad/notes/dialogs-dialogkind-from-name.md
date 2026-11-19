---
id: dialogs-dialogkind-from-name
kind: doc
target_file: src/dialogs.rs
line_hint: 64
author: 
created: 
title: "DialogKind::from_name"
tags: []
signature: "pub fn from_name (name : & str) -> Self"
---

Parses a dialog kind from its inspector/PascalCase name.

Unknown values default to `DialogKind::Information`.

# Examples

```rust
use crawrad_runtime::dialogs::DialogKind;

assert_eq!(DialogKind::from_name("mtError"), DialogKind::Error);
```
