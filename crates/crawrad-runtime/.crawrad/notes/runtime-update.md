---
id: runtime-update
kind: doc
target_file: src/lib.rs
line_hint: 180
author: 
created: 
title: "Runtime::update"
tags: []
signature: "fn update (& mut self , doc : & Document , touched : & BTreeSet < NodeId >) -> Result < () , RuntimeError >"
---

Applies an incremental change for touched nodes.

Applies an incremental change. `touched` comes from `crawrad_doc::Applied`,
so the cost is proportional to what changed, not the size of the form.

# Examples

```rust
use std::collections::BTreeSet;
use crawrad_ir::Document;
use crawrad_runtime::Runtime;
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
assert!(rt.update(&doc, &BTreeSet::new()).is_ok());
```

# Errors

Returns [`RuntimeError`] if applying the incremental change fails.
