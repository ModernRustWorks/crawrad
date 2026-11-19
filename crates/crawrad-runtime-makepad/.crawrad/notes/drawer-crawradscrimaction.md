---
id: drawer-crawradscrimaction
kind: doc
target_file: src/drawer.rs
line_hint: 38
author: 
created: 
title: "CrawradScrimAction"
tags: []
signature: "pub enum CrawradScrimAction"
---

What [`CrawradScrim`] reports.

# Examples

```rust
use crawrad_runtime_makepad::drawer::CrawradScrimAction;

let action = CrawradScrimAction::Clicked;
assert!(matches!(action, CrawradScrimAction::Clicked));
```
