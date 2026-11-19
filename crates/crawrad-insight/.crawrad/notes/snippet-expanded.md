---
id: snippet-expanded
kind: doc
target_file: src/snippet.rs
line_hint: 16
author: 
created: 
title: "Expanded"
tags: []
signature: "pub struct Expanded"
---

Expanded snippet text along with byte ranges for each tabstop.

The text to write, and its tabstops in tab order — `$1`, `$2`… then `$0`
(the end of the text when the snippet has none) — as byte ranges.

# Examples

```rust
use crawrad_insight::snippet::Expanded;

let exp = Expanded { text: "val".into(), stops: vec![(0, 3)] };
assert_eq!(exp.stops.len(), 1);
```
