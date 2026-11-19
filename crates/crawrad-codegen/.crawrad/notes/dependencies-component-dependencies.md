---
id: dependencies-component-dependencies
kind: doc
target_file: src/dependencies.rs
line_hint: 202
author: 
created: 
title: "component_dependencies"
tags: []
signature: "pub fn component_dependencies (doc : & crawrad_ir :: Document , alvo : AlvoGerador) -> Vec < Dependencia >"
---

The crates the components ON this form need beyond the target's own.

The target's list answers "what does any form of this target use"; this one
answers "what does THIS form use". A grid bound to a `Connection` reads its
table through the grid crates, and on egui paints with the egui runtime's
data grid. The IDE adds these on every Generate, like third-party crates,
because binding a grid happens long after the project was created.

# Examples

```rust
use crawrad_codegen::{AlvoGerador, component_dependencies};
use crawrad_ir::Document;

let doc = Document::new("MainForm", "Form");
let deps = component_dependencies(&doc, AlvoGerador::Makepad);
assert!(deps.is_empty());
```
