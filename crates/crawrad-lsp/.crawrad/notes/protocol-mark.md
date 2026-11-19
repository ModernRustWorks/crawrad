---
id: protocol-mark
kind: doc
target_file: src/protocol.rs
line_hint: 422
author: 
created: 
title: "Mark"
tags: []
signature: "pub struct Mark"
---

What the editor draws for one range of a file's diagnostics (#361).

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
