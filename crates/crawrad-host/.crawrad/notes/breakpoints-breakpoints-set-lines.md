---
id: breakpoints-breakpoints-set-lines
kind: doc
target_file: src/breakpoints.rs
line_hint: 52
author: 
created: 
title: "Breakpoints::set_lines"
tags: []
signature: "pub fn set_lines (& mut self , file : & Path , lines : impl IntoIterator < Item = u32 >)"
---

Replaces the breakpoints of `file`.

Replaces the breakpoints of `file`. Line `0` does not exist and is dropped.

A logpoint's message follows its line. An edit above it moves every
line below by the same amount, so the lines that went away and the
lines that appeared pair up in order: the message of the first line
gone goes to the first line new. A line removed outright takes its
message with it.

# Examples

```rust
use std::path::Path;
use crawrad_host::Breakpoints;

let mut bp = Breakpoints::default();
bp.set_lines(Path::new("src/lib.rs"), [1, 5]);
assert_eq!(bp.lines(Path::new("src/lib.rs")), vec![1, 5]);
```
