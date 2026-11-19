---
id: targets-shared-peca-edges
kind: doc
target_file: src/targets/shared.rs
line_hint: 191
author: 
created: 
title: "PECA_EDGES"
tags: []
signature: "pub const PECA_EDGES : & str = include_str ! (\"../../../crawrad-component/src/pieces/edges.rs\")"
---

The text of the `Edges` piece, ready to go to the user's project.

# Examples

```rust
use crawrad_codegen::targets::shared::PECA_EDGES;

assert!(PECA_EDGES.contains("struct Edges"));
```
