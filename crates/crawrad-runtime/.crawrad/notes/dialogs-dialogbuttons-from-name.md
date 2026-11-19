---
id: dialogs-dialogbuttons-from-name
kind: doc
target_file: src/dialogs.rs
line_hint: 133
author: 
created: 
title: "DialogButtons::from_name"
tags: []
signature: "pub fn from_name (name : & str) -> Self"
---

Parses dialog buttons from their inspector/PascalCase name.

Unknown values default to `DialogButtons::Ok`.

# Examples

```rust
use crawrad_runtime::dialogs::DialogButtons;

assert_eq!(DialogButtons::from_name("mbYesNo"), DialogButtons::YesNo);
```
