---
id: runtime
kind: doc
target_file: src/lib.rs
line_hint: 169
author: 
created: 
title: "Runtime"
tags: []
signature: "pub trait Runtime"
---

What knows how to turn a [`Document`] into something on screen.

# Examples

```rust
use std::collections::BTreeSet;
use crawrad_ir::{Document, NodeId};
use crawrad_runtime::{GuideLine, Mode, Runtime, RuntimeError, RuntimeEvent, Scale, ViewportPx};

struct MockRt;
impl Runtime for MockRt {
    fn target(&self) -> &'static str { "mock" }
    fn classes(&self) -> &'static [&'static str] { &[] }
    fn open(&mut self, _doc: &Document) -> Result<(), RuntimeError> { Ok(()) }
    fn update(&mut self, _doc: &Document, _touched: &BTreeSet<NodeId>) -> Result<(), RuntimeError> { Ok(()) }
    fn set_viewport(&mut self, _vp: ViewportPx, _scale: Scale) {}
    fn set_selection(&mut self, _sel: &[NodeId]) {}
    fn set_marquee(&mut self, _rect: Option<crawrad_ir::Rect>) {}
    fn set_guides(&mut self, _guides: &[GuideLine]) {}
    fn drain_events(&mut self) -> Vec<RuntimeEvent> { Vec::new() }
    fn close(&mut self) {}
}

let rt = MockRt;
assert_eq!(rt.target(), "mock");
```
