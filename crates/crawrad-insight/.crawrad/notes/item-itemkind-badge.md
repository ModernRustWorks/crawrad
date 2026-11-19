---
id: item-itemkind-badge
kind: doc
target_file: src/item.rs
line_hint: 31
author: 
created: 
title: "ItemKind::badge"
tags: []
signature: "pub fn badge (& self) -> & 'static str"
---

Returns the Codicons font glyph associated with this item kind.

The row's mark: VS Code's Codicons symbol for the kind
(`symbol-field`, `symbol-method`, `symbol-class`, `symbol-keyword`,
`symbol-snippet`, `loading`), code points read from `codicon.ttf`.
The shell draws them in a family where the code font falls back to
Codicons for these points only; the geometric characters used before
(`⬡ ◇ ⧉ ▫ ▪`) are in neither font and drew as empty boxes.

# Examples

```rust
use crawrad_insight::item::ItemKind;

assert!(!ItemKind::Field.badge().is_empty());
```
