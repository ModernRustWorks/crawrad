---
id: protocol-range
kind: doc
target_file: src/protocol.rs
line_hint: 24
author: 
created: 
title: "Range"
tags: []
signature: "pub struct Range"
---

A range in a text document expressed as (zero-based) start and end positions.

# Examples

```rust
use crawrad_lsp::protocol::{Position, Range};

let range = Range::new(Position::new(0, 0), Position::new(0, 5));
assert_eq!(range.end.character, 5);
```
