---
id: generator-no-member
kind: doc
target_file: src/generator.rs
line_hint: 83
author: 
created: 
title: "NO_MEMBER"
tags: []
signature: "pub const NO_MEMBER : & str = \"no form open — a form lives in <member>/src/\""
---

There is no form open, and a form lives in `<member>/src/`.

A named reason and not a literal at the call site: both shells say it, and
the day the sentence improves it improves once.

# Examples

```rust
use crawrad_project::generator::NO_MEMBER;

assert!(NO_MEMBER.contains("no form open"));
```
