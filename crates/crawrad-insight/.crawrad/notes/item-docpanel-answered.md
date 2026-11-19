---
id: item-docpanel-answered
kind: doc
target_file: src/item.rs
line_hint: 284
author: 
created: 
title: "DocPanel::answered"
tags: []
signature: "pub fn answered (& mut self , ticket : u64 , doc : Option < String >) -> bool"
---

The resolve for `ticket` answered; whether it was the one awaited.

# Examples

```rust
use crawrad_insight::item::DocPanel;

let mut panel = DocPanel::default();
assert!(!panel.answered(1, Some("doc".into())));
```
