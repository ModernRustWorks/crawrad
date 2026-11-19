---
id: build-progress-encode-line
kind: doc
target_file: src/build_progress.rs
line_hint: 314
author: 
created: 
title: "encode_line"
tags: []
signature: "pub fn encode_line (pipe : crawrad_os :: process :: Pipe , line : & str) -> String"
---

Encodes a pipe identifier and streamed output line into transport text.

# Examples

```rust
use crawrad_os::process::Pipe;
use crawrad_project::build_progress::encode_line;

assert_eq!(encode_line(Pipe::Stdout, "hello"), "o hello");
assert_eq!(encode_line(Pipe::Stderr, "warning"), "e warning");
```
