---
id: encoding-positionencoding-offered
kind: doc
target_file: src/encoding.rs
line_hint: 27
author: 
created: 
title: "PositionEncoding::OFFERED"
tags: []
signature: "pub const OFFERED : [& 'static str ; 2] = [\"utf-8\" , \"utf-16\"]"
---

What the client offers, best first.

# Examples

```rust
use crawrad_lsp::encoding::PositionEncoding;

assert_eq!(PositionEncoding::OFFERED[0], "utf-8");
```
