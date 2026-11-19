---
id: properties-outcome
kind: doc
target_file: src/properties.rs
line_hint: 22
author: 
created: 
title: "Outcome"
tags: []
signature: "pub struct Outcome"
---

The keys of one call, as they came out.

# Examples

```rust
use crawrad_mcp::properties::Outcome;

let mut outcome = Outcome::default();
outcome.apply("caption");
assert!(!outcome.failed());
```
