---
id: protocol-position
kind: doc
target_file: src/protocol.rs
line_hint: 6
author: 
created: 
title: "Position"
tags: []
signature: "pub struct Position"
---

Position in a text document with zero-based line and character offsets.

Position in a text document expressed as zero-based line and zero-based character offset.

# Examples

```rust
use crawrad_lsp::protocol::Position;

let pos = Position::new(10, 4);
assert_eq!(pos.line, 10);
assert_eq!(pos.character, 4);
```
