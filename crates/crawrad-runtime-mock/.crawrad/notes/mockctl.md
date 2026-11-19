---
id: mockctl
kind: doc
target_file: src/lib.rs
line_hint: 26
author: 
created: 
title: "MockCtl"
tags: []
signature: "pub struct MockCtl"
---

A component as the mock knows it: class, rectangle, and text.

# Examples

```rust
use crawrad_runtime_mock::MockCtl;
use crawrad_ir::Rect;

let ctl = MockCtl {
    class: "Button".into(),
    rect: Rect::new(0.0, 0.0, 80.0, 24.0),
    text: "OK".into(),
    visible: true,
    enabled: true,
};
assert_eq!(ctl.class, "Button");
```
