---
id: references-references
kind: doc
target_file: src/references.rs
line_hint: 88
author: 
created: 
title: "References"
tags: []
signature: "pub struct References"
---

The panel's state: the symbol, the groups, and the request in flight.

# Examples

```rust
use crawrad_host::references::References;

let refs = References::default();
assert_eq!(refs.total(), 0);
```
