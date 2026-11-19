---
id: highlight-highlightworker-poll
kind: doc
target_file: src/highlight.rs
line_hint: 190
author: 
created: 
title: "HighlightWorker::poll"
tags: []
signature: "pub fn poll (& self , file : & Path , version : i32 , text : & str , caret : Caret ,) -> Option < Vec < Occurrence > >"
---

Returns matching occurrences to draw on `text`, if ready.

The occurrences to draw — placed on `text` — when an answer for THIS
file, version and caret arrived; anything older is dropped.

# Examples

```rust
use crawrad_lsp::highlight::HighlightWorker;
use std::path::Path;

let worker = HighlightWorker::new();
let occs = worker.poll(Path::new("src/lib.rs"), 1, "fn main() {}", (0, 0));
assert_eq!(occs, None);
```
