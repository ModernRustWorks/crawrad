---
id: item-itemkind-hex-colour
kind: doc
target_file: src/item.rs
line_hint: 59
author: 
created: 
title: "ItemKind::hex_colour"
tags: []
signature: "pub fn hex_colour (& self) -> String"
---

The icon's colour as `#rrggbb`, for the MCP.

# Examples

```rust
use crawrad_insight::item::ItemKind;

assert_eq!(ItemKind::Field.hex_colour(), "#7cb46b");
```
