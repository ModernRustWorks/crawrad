---
id: protocol-locations-from
kind: doc
target_file: src/protocol.rs
line_hint: 49
author: 
created: 
title: "locations_from"
tags: []
signature: "pub fn locations_from (value : serde_json :: Value) -> Vec < Location >"
---

Flattens an LSP definition-like response value into a list of locations.

A definition-like answer as a flat list of places. The server may send
`null`, one `Location`, `Location[]` or — when the client says it takes
links, as this one does — `LocationLink[]`; a link lands on its
`targetSelectionRange` (the name), not the whole item.

# Examples

```rust
use crawrad_lsp::protocol::locations_from;
use serde_json::json;

let locs = locations_from(json!(null));
assert!(locs.is_empty());
```
