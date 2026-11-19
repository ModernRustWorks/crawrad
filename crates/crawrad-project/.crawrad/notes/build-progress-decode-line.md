---
id: build-progress-decode-line
kind: doc
target_file: src/build_progress.rs
line_hint: 322
author: 
created: 
title: "decode_line"
tags: []
signature: "pub fn decode_line (text : & str) -> Option < (crawrad_os :: process :: Pipe , & str) >"
---

Decodes transport text into its pipe identifier and output line.

# Examples

```rust
use crawrad_os::process::Pipe;
use crawrad_project::build_progress::decode_line;

assert_eq!(decode_line("o hello"), Some((Pipe::Stdout, "hello")));
assert_eq!(decode_line("e warning"), Some((Pipe::Stderr, "warning")));
assert_eq!(decode_line("unknown"), None);
```
