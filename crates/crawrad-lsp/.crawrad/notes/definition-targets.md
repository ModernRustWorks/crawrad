---
id: definition-targets
kind: doc
target_file: src/definition.rs
line_hint: 104
author: 
created: 
title: "targets"
tags: []
signature: "pub fn targets (places : Vec < Location > , enc : crate :: encoding :: PositionEncoding , read : & mut dyn FnMut (& Path) -> Option < String > ,) -> Vec < Target >"
---

Converts protocol locations into editor target positions.

The server's places, in the editor's unit. A file that cannot be read
keeps the line and lands on the line's start.

# Examples

```rust
use crawrad_lsp::definition::targets;
use crawrad_lsp::encoding::PositionEncoding;

let res = targets(Vec::new(), PositionEncoding::Utf8, &mut |_| None);
assert!(res.is_empty());
```
