---
id: runtime-classes
kind: doc
target_file: src/lib.rs
line_hint: 174
author: 
created: 
title: "Runtime::classes"
tags: []
signature: "fn classes (& self) -> & 'static [& 'static str]"
---

The classes that this adapter actually renders as child controls.

The classes that this adapter **actually** renders, as a child control
of the form.

Without this list, a component added to the registry that did not receive
rendering here becomes a rectangle with the name inside — and the hole only
appears when someone drags the component onto the screen. With it, each adapter's
test checks the registry and the registry checks the adapter.

Excluded: `Form`, which is the surface and not a child, and
non-visuals, which become icons in the designer and nothing in the application.

# Examples

```rust
use crawrad_runtime::Runtime;
# use std::collections::BTreeSet;
# use crawrad_ir::{Document, NodeId};
# use crawrad_runtime::{GuideLine, RuntimeError, RuntimeEvent, Scale, ViewportPx};
# struct Mock;
# impl Runtime for Mock {
#     fn target(&self) -> &'static str { "mock" }
#     fn classes(&self) -> &'static [&'static str] { &["Label"] }
#     fn open(&mut self, _: &Document) -> Result<(), RuntimeError> { Ok(()) }
#     fn update(&mut self, _: &Document, _: &BTreeSet<NodeId>) -> Result<(), RuntimeError> { Ok(()) }
#     fn set_viewport(&mut self, _: ViewportPx, _: Scale) {}
#     fn set_selection(&mut self, _: &[NodeId]) {}
#     fn set_marquee(&mut self, _: Option<crawrad_ir::Rect>) {}
#     fn set_guides(&mut self, _: &[GuideLine]) {}
#     fn drain_events(&mut self) -> Vec<RuntimeEvent> { Vec::new() }
#     fn close(&mut self) {}
# }

let rt = Mock;
assert_eq!(rt.classes(), &["Label"]);
```
