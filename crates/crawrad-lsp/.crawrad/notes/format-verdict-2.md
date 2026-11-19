---
id: format-verdict-2
kind: doc
target_file: src/format.rs
line_hint: 105
author: 
created: 
title: "verdict"
tags: []
signature: "pub fn verdict (outcome : Outcome , still_same : bool , text : & str , enc : crate :: encoding :: PositionEncoding ,) -> Verdict"
---

Evaluates formatting outcome against current file status.

The answer, judged: dropped when the file changed while the server
worked (`still_same` false), "already formatted" when it changes
nothing — so the file never turns dirty for nothing.

# Examples

```rust
use crawrad_lsp::encoding::PositionEncoding;
use crawrad_lsp::format::{verdict, Outcome, Verdict};

let v = verdict(Outcome::Edits(Vec::new()), true, "fn main() {}", PositionEncoding::Utf8);
assert_eq!(v, Verdict::Say("Already formatted"));
```
