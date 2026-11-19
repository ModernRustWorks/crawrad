---
id: targets-makepad-layout-active-page
kind: doc
target_file: src/targets/makepad/layout.rs
line_hint: 45
author: 
created: 
title: "active_page"
tags: []
signature: "pub fn active_page (n : & Node) -> usize"
---

Retrieves the active page index for a PageControl.

# Examples

```rust
use crawrad_codegen::targets::makepad::layout::active_page;
use crawrad_ir::Node;

let node = Node::new("pgc", "PageControl");
assert_eq!(active_page(&node), 0);
```
