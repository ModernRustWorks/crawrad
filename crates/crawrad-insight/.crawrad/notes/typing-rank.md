---
id: typing-rank
kind: doc
target_file: src/typing.rs
line_hint: 105
author: 
created: 
title: "rank"
tags: []
signature: "pub fn rank < 'a > (labels : impl IntoIterator < Item = & 'a str > , query : & str) -> Vec < usize >"
---

`labels` that answer `query`, best first; equal scores keep their order.

# Examples

```rust
use crawrad_insight::typing::rank;

let labels = vec!["swapcase", "split"];
let ranked = rank(labels.iter().copied(), "sp");
assert_eq!(ranked, vec![1, 0]);
```
