---
id: asker-asker-take
kind: doc
target_file: src/asker.rs
line_hint: 51
author: 
created: 
title: "Asker::take"
tags: []
signature: "pub fn take (& self , ticket : u64) -> Option < T >"
---

The answer for `ticket`, if it arrived; any other is dropped.

# Examples

```rust
use crawrad_lsp::asker::Asker;

let asker = Asker::<i32>::new("worker");
assert_eq!(asker.take(99), None);
```
