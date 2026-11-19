---
id: dialogs-dialogkind-default-title
kind: doc
target_file: src/dialogs.rs
line_hint: 31
author: 
created: 
title: "DialogKind::default_title"
tags: []
signature: "pub const fn default_title (& self) -> & 'static str"
---

Default window title according to dialog kind.

# Examples

```rust
use crawrad_runtime::dialogs::DialogKind;

assert_eq!(DialogKind::Warning.default_title(), "Warning");
```
