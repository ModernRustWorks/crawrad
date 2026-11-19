---
id: item-docpanel
kind: doc
target_file: src/item.rs
line_hint: 233
author: 
created: 
title: "DocPanel"
tags: []
signature: "pub struct DocPanel"
---

Documentation panel displayed alongside the completion list.

The documentation panel beside the completion list (#360): the item it
is about, its text, and the resolve on the way.

# Examples

```rust
use crawrad_insight::item::DocPanel;

let panel = DocPanel::default();
assert!(panel.text.is_none());
```
