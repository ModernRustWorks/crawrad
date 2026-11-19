---
id: source-without-cr
kind: doc
target_file: src/source.rs
line_hint: 134
author: 
created: 
title: "without_cr"
tags: []
signature: "pub fn without_cr (text : & str) -> String"
---

The text as the VIEW shows it: without the `\r` of CRLF.

# This is drawing, not editing

Makepad's `CodeView` draws `\r` as a **missing glyph box** — a
square at the end of every line, measured on screen on 2026-09-16 with the
sample app unit in CRLF. Any file coming from a `git
clone` on Windows comes out like this.

Nothing is saved: this shell does not edit code, and the file on disk remains
with the line ending its owner chose — which generation, incidentally, now
respects (`crawrad_codegen::rxp::marca_em`). It is the same choice every editor
makes: a modern editor shows CRLF without drawing `\r`, and states what it is in the status bar.

# Examples

```rust
use crawrad_project::source::without_cr;

assert_eq!(without_cr("a\r\nb\r\n"), "a\nb\n");
```
