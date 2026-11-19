---
id: encoding-encoding
kind: doc
target_file: src/encoding.rs
line_hint: 65
author: 
created: 
title: "Encoding"
tags: []
signature: "pub struct Encoding"
---

How a file was written: the mark it carried and how it ended its lines.

# Examples

```rust
use crawrad_project::encoding::{Encoding, LineEnding};

let enc = Encoding { bom: false, line_ending: LineEnding::Lf };
assert_eq!(enc.label(), "UTF-8  LF");
```
