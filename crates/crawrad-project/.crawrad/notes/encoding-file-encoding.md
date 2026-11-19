---
id: encoding-file-encoding
kind: doc
target_file: src/encoding.rs
line_hint: 108
author: 
created: 
title: "file_encoding"
tags: []
signature: "pub fn file_encoding (path : Option < & std :: path :: Path >) -> Encoding"
---

How a file is written on disk, read from its raw bytes.

Returns `Encoding::default()` if the path is `None` or reading the file fails.

# Examples

```rust
use crawrad_project::encoding::{file_encoding, LineEnding};

let enc = file_encoding(None);
assert_eq!(enc.line_ending, LineEnding::Lf);
```
