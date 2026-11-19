---
id: target-name
kind: doc
target_file: src/lib.rs
line_hint: 148
author: 
created: 
title: "ALVO"
tags: []
signature: "pub const ALVO : & str = \"makepad\""
---

Target identifier for this runtime — matches property prefix from ADR 0002.

# Examples

```rust
use crawrad_runtime_makepad::ALVO;

assert_eq!(ALVO, "makepad");
```
