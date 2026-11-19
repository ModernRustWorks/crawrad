---
id: properties-outcome-failed
kind: doc
target_file: src/properties.rs
line_hint: 38
author: 
created: 
title: "Outcome::failed"
tags: []
signature: "pub fn failed (& self) -> bool"
---

Nothing applied and something refused: the call failed.

# Examples

```rust
use crawrad_mcp::properties::Outcome;

let mut outcome = Outcome::default();
outcome.refuse("invalid", "unknown key");
assert!(outcome.failed());
```
