---
id: dock-side-by-side
kind: doc
target_file: src/dock.rs
line_hint: 1
author: 
created: 
title: "side_by_side"
tags: []
signature: "pub fn side_by_side (a : LiveId , b : LiveId , align : SplitterAlign) -> DockItem"
---

Two dock items side by side, the split measured by `align`.

# Examples

```
use crawrad_runtime_makepad::dock::side_by_side;
use crawrad_runtime_makepad::makepad_widgets::{LiveId, splitter::SplitterAlign};

let splitter = side_by_side(LiveId::from_str("left"), LiveId::from_str("center"), SplitterAlign::FromA(300.0));
assert!(matches!(splitter, crawrad_runtime_makepad::makepad_widgets::DockItem::Splitter { .. }));
```
