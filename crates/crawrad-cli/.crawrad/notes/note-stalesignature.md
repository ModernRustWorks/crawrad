---
id: note-stalesignature
kind: doc
target_file: src/note.rs
line_hint: 54
author: 
created: 
title: "StaleSignature"
tags: []
signature: "pub struct StaleSignature"
---

A note whose recorded signature differs from the item's current signature.

# Examples

```rust
use crawrad_cli::note::StaleSignature;

let stale = StaleSignature {
    id: "calc".to_string(),
    file: "src/calc.rs".to_string(),
    line: 10,
    message: "signature changed".to_string(),
};
assert_eq!(stale.id, "calc");
```
