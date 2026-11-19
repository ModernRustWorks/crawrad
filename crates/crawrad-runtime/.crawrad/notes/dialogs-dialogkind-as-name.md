---
id: dialogs-dialogkind-as-name
kind: doc
target_file: src/dialogs.rs
line_hint: 76
author: 
created: 
title: "DialogKind::as_name"
tags: []
signature: "pub const fn as_name (& self) -> & 'static str"
---

Returns the canonical inspector enum identifier.

# Examples

```rust
use crawrad_runtime::dialogs::DialogKind;

assert_eq!(DialogKind::Error.as_name(), "mtError");
```
