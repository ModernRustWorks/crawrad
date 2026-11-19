---
id: grid-footer
kind: doc
target_file: src/grid.rs
line_hint: 451
author: 
created: 
title: "Footer"
tags: []
signature: "pub struct Footer"
---

Formatted footer bar describing position, status, ordering, and latency.

Formatted footer bar describing viewport position, status, ordering, and query latency.

# Examples

```rust
use crawrad_runtime::grid::Footer;

let footer = Footer {
    position: "Row 1 of 10".into(),
    state: "",
    order: "".into(),
    timing: "1.2 ms".into(),
};
assert_eq!(footer.position, "Row 1 of 10");
```
