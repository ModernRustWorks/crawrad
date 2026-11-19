---
id: dialogs-dialogbuttons-as-name
kind: doc
target_file: src/dialogs.rs
line_hint: 144
author: 
created: 
title: "DialogButtons::as_name"
tags: []
signature: "pub const fn as_name (& self) -> & 'static str"
---

Returns the canonical inspector enum identifier.

# Examples

```rust
use crawrad_runtime::dialogs::DialogButtons;

assert_eq!(DialogButtons::YesNo.as_name(), "mbYesNo");
```
