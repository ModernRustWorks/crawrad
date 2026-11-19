---
id: format-verdict
kind: doc
target_file: src/format.rs
line_hint: 96
author: 
created: 
title: "Verdict"
tags: []
signature: "pub enum Verdict"
---

What to do with an answer.

# Examples

```rust
use crawrad_lsp::format::Verdict;

let verdict = Verdict::Say("Already formatted");
assert_eq!(verdict, Verdict::Say("Already formatted"));
```
