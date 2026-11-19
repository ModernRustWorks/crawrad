---
id: document-counters-2
kind: doc
target_file: src/document.rs
line_hint: 165
author: 
created: 
title: "counters"
tags: []
signature: "pub fn counters (doc : Option < & Doc >) -> Counters"
---

The three cells, of the open document.

# Examples

```rust
use crawrad_host::document::counters;

let c = counters(None);
assert_eq!(c.nodes, "—");
```
