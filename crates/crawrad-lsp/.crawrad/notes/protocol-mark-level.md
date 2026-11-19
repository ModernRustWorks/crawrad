---
id: protocol-mark-level
kind: doc
target_file: src/protocol.rs
line_hint: 464
author: 
created: 
title: "Mark::level"
tags: []
signature: "pub fn level (& self) -> usize"
---

0 a hint (no squiggle), 1 a warning, 2 an error.

# Examples

```rust
use crawrad_lsp::protocol::{Mark, Position, Range};

let mark = Mark {
    range: Range::new(Position::new(0, 0), Position::new(0, 1)),
    error: true,
    hint: false,
    faded: false,
    struck: false,
};
assert_eq!(mark.level(), 2);
```
