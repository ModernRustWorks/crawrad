---
id: item-fit
kind: doc
target_file: src/item.rs
line_hint: 167
author: 
created: 
title: "fit"
tags: []
signature: "pub fn fit (text : & str , room : f64 , mut width : impl FnMut (& str) -> f64) -> String"
---

Truncates text to fit within a given pixel room using ellipsis.

Truncates `text` so it fits within `room` using `width`.
Returns `text` when `width(text) <= room`; otherwise the longest
char-boundary prefix `p` with `width(&format!("{p}…")) <= room`,
followed by `…`; `""` when even `"…"` does not fit.

# Examples

```rust
use crawrad_insight::item::fit;

assert_eq!(fit("hello world", 100.0, |_| 10.0), "hello world");
```
