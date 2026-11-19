---
id: encoding-encoding-label
kind: doc
target_file: src/encoding.rs
line_hint: 97
author: 
created: 
title: "Encoding::label"
tags: []
signature: "pub fn label (& self) -> & 'static str"
---

The encoding and the line ending, as the status strip writes them.

# Examples

```rust
use crawrad_project::encoding::{Encoding, LineEnding};

let enc = Encoding { bom: true, line_ending: LineEnding::CrLf };
assert_eq!(enc.label(), "UTF-8 BOM  CRLF");
```
