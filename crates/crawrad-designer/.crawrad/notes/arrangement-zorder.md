---
id: arrangement-zorder
kind: doc
target_file: src/arrangement.rs
line_hint: 243
author: 
created: 
title: "ZOrder"
tags: []
signature: "pub enum ZOrder"
---

Where to send the component in the stacking order.

# Examples

```rust
use crawrad_designer::arrangement::ZOrder;

assert_eq!(ZOrder::Front.label(), "Bring to front");
```
