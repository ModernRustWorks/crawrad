---
id: runtime-set-viewport
kind: doc
target_file: src/lib.rs
line_hint: 183
author: 
created: 
title: "Runtime::set_viewport"
tags: []
signature: "fn set_viewport (& mut self , vp : ViewportPx , scale : Scale)"
---

Where the design surface lives inside the IDE window.

# Examples

```rust
use crawrad_runtime::{Runtime, Scale, ViewportPx};
# use std::collections::BTreeSet;
# use crawrad_ir::{Document, NodeId};
# use crawrad_runtime::{GuideLine, RuntimeError, RuntimeEvent};
# struct Mock;
# impl Runtime for Mock {
#     fn target(&self) -> &'static str { "mock" }
#     fn classes(&self) -> &'static [&'static str] { &[] }
#     fn open(&mut self, _: &Document) -> Result<(), RuntimeError> { Ok(()) }
#     fn update(&mut self, _: &Document, _: &BTreeSet<NodeId>) -> Result<(), RuntimeError> { Ok(()) }
#     fn set_viewport(&mut self, _: ViewportPx, _: Scale) {}
#     fn set_selection(&mut self, _: &[NodeId]) {}
#     fn set_marquee(&mut self, _: Option<crawrad_ir::Rect>) {}
#     fn set_guides(&mut self, _: &[GuideLine]) {}
#     fn drain_events(&mut self) -> Vec<RuntimeEvent> { Vec::new() }
#     fn close(&mut self) {}
# }

let mut rt = Mock;
rt.set_viewport(ViewportPx { x: 0, y: 0, w: 800, h: 600 }, Scale(1.0));
```
