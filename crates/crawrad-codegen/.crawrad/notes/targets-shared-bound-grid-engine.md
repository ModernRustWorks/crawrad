---
id: targets-shared-bound-grid-engine
kind: doc
target_file: src/targets/shared.rs
line_hint: 109
author: 
created: 
title: "bound_grid_engine"
tags: []
signature: "pub fn bound_grid_engine (doc : & crawrad_ir :: Document , node : & crawrad_ir :: Node ,) -> Result < Option < String > , crate :: Erro >"
---

The field of the `Connection` a `Grid` reads its rows from.

The field of the `Connection` a `Grid` reads its rows from, or `None` for a
grid that shows its own cells.

A `DataSource` pointing at a component that no longer exists is `None` as
well: the form keeps the stale reference, and the app is born unbound
rather than naming a component that is gone.

# Examples

```rust
use crawrad_codegen::targets::shared::bound_grid_engine;
use crawrad_ir::{Document, Node};

let doc = Document::new("form", "Form");
let node = Node::new("grid1", "Grid");
let engine = bound_grid_engine(&doc, &node).unwrap();
assert!(engine.is_none());
```

# Errors

Returns an error if the referenced data engine name is not a valid identifier.
