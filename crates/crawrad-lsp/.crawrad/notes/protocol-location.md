---
id: protocol-location
kind: doc
target_file: src/protocol.rs
line_hint: 40
author: 
created: 
title: "Location"
tags: []
signature: "pub struct Location"
---

Represents a location inside a resource, such as a line inside a text file.

# Examples

```rust
use crawrad_lsp::protocol::{Location, Position, Range};

let loc = Location {
    uri: "file:///src/lib.rs".to_string(),
    range: Range::new(Position::new(0, 0), Position::new(0, 5)),
};
assert_eq!(loc.range.start.line, 0);
```
