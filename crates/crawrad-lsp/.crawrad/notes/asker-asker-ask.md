---
id: asker-asker-ask
kind: doc
target_file: src/asker.rs
line_hint: 46
author: 
created: 
title: "Asker::ask"
tags: []
signature: "pub fn ask (& self , ticket : u64 , work : impl FnOnce () -> T + Send + 'static) -> bool"
---

Hands `work` over under `ticket`.

# Examples

```rust
use crawrad_lsp::asker::Asker;

let asker = Asker::<i32>::new("worker");
assert!(asker.ask(1, || 42));
```
