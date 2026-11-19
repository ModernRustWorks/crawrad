---
id: process-pipe
kind: doc
target_file: src/process.rs
line_hint: 222
author: 
created: 
title: "Pipe"
tags: []
signature: "pub enum Pipe"
---

Which pipe a streamed line came from.

# Examples

```rust
use crawrad_os::process::Pipe;

assert_eq!(Pipe::Stdout, Pipe::Stdout);
```
