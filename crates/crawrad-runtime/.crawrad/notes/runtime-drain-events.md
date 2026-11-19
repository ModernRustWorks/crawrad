---
id: runtime-drain-events
kind: doc
target_file: src/lib.rs
line_hint: 223
author: 
created: 
title: "Runtime::drain_events"
tags: []
signature: "fn drain_events (& mut self) -> Vec < RuntimeEvent >"
---

Input events accumulated since the last call.

# Examples

```rust
use crawrad_runtime::Runtime;
# use std::collections::BTreeSet;
# use crawrad_ir::{Document, NodeId};
# use crawrad_runtime::{GuideLine, RuntimeError, RuntimeEvent, Scale, ViewportPx};
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
assert!(rt.drain_events().is_empty());
```
