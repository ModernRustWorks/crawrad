---
id: item-docpanel-select
kind: doc
target_file: src/item.rs
line_hint: 255
author: 
created: 
title: "DocPanel::select"
tags: []
signature: "pub fn select (& mut self , item : Option < & ItemSugestao >) -> DocStep"
---

Updates documentation selection and returns the required UI step.

The list's selection is now `item` (`None`: the list closed). The
documentation sent with the list shows at once; otherwise the item is
resolved, and only the newest ticket's answer is taken.

# Examples

```rust
use crawrad_insight::item::{DocPanel, DocStep};

let mut panel = DocPanel::default();
// Nothing selected yet and the list still closed: nothing to redraw.
assert_eq!(panel.select(None), DocStep::Same);
```
