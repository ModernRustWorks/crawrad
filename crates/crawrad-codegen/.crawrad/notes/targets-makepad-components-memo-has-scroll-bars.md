---
id: targets-makepad-components-memo-has-scroll-bars
kind: doc
target_file: src/targets/makepad/components/memo.rs
line_hint: 187
author: 
created: 
title: "has_scroll_bars"
tags: []
signature: "pub fn has_scroll_bars (node : & Node) -> bool"
---

Checks whether this Memo node has scroll bars enabled.

Whether this `Memo` node has scroll bars enabled (anything other than `ssNone`).

# Examples

```rust
use crawrad_codegen::targets::makepad::components::memo::has_scroll_bars;
use crawrad_ir::Node;

let node = Node::new("memo", "Memo");
assert!(!has_scroll_bars(&node));
```
