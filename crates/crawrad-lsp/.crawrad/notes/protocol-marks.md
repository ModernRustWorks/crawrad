---
id: protocol-marks
kind: doc
target_file: src/protocol.rs
line_hint: 436
author: 
created: 
title: "marks"
tags: []
signature: "pub fn marks (diagnostics : & [Diagnostic]) -> Vec < Mark >"
---

Merges diagnostics covering identical ranges into single editor marks.

One mark per range: the editor keeps one decoration where one starts, so
diagnostics on the same range are merged — the most severe squiggle, the
tags of all of them. Measured on rust-analyzer: rustc's `unused_variables`
comes as a Warning with tag 1 AND a Hint without tags on the same range;
unmerged, the Hint came last and took the fade away.

# Examples

```rust
use crawrad_lsp::protocol::marks;

let m = marks(&[]);
assert!(m.is_empty());
```
