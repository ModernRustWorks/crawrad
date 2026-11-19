---
id: dialogs-dialogkind-icon-symbol
kind: doc
target_file: src/dialogs.rs
line_hint: 42
author: 
created: 
title: "DialogKind::icon_symbol"
tags: []
signature: "pub const fn icon_symbol (& self) -> & 'static str"
---

Icon symbol for the dialog (Unicode gliph or icon tag).

# Examples

```rust
use crawrad_runtime::dialogs::DialogKind;

assert_eq!(DialogKind::Warning.icon_symbol(), "⚠");
```
