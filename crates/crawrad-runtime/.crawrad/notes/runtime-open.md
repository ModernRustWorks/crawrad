---
id: runtime-open
kind: doc
target_file: src/lib.rs
line_hint: 177
author: 
created: 
title: "Runtime::open"
tags: []
signature: "fn open (& mut self , doc : & Document) -> Result < () , RuntimeError >"
---

(Re)constructs the entire form from the document.

# Examples

```rust
use crawrad_ir::Document;
use crawrad_runtime::Runtime;
# use std::collections::BTreeSet;
# use crawrad_ir::NodeId;
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
let doc = Document::new("form", "Form");
assert!(rt.open(&doc).is_ok());
```

# Errors

Returns [`RuntimeError`] if constructing the form or its controls fails.
