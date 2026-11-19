---
id: references-spans
kind: doc
target_file: src/references.rs
line_hint: 118
author: 
created: 
title: "spans"
tags: []
signature: "pub fn spans (places : Vec < Location > , enc : crate :: encoding :: PositionEncoding , read : & mut dyn FnMut (& Path) -> Option < String > ,) -> Vec < Span >"
---

Converts server locations into editor spans using file text.

The server's places as [`Span`]s, in its order. `read` gives a file's
text (it is asked once per file); a range that runs past its line ends
at the line's end; a file that cannot be read keeps the line, empty.

# Examples

```rust
use crawrad_lsp::encoding::PositionEncoding;
use crawrad_lsp::protocol::{Location, Position, Range};
use crawrad_lsp::references::spans;
use std::path::Path;

let loc = Location {
    uri: "file:///test/main.rs".to_string(),
    range: Range {
        start: Position { line: 0, character: 3 },
        end: Position { line: 0, character: 7 },
    },
};
let mut read = |p: &Path| Some("fn main() {}".to_string());
let result = spans(vec![loc], PositionEncoding::Utf8, &mut read);
assert_eq!(result.len(), 1);
assert_eq!(result[0].start, 3);
assert_eq!(result[0].end, 7);
```
