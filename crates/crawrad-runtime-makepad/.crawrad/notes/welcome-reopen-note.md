---
id: welcome-reopen-note
kind: doc
target_file: src/welcome.rs
line_hint: 253
author: 
created: 
title: "reopen_note"
tags: []
signature: "pub fn reopen_note (reopen : bool) -> & 'static str"
---

The note under the `On start` box, which says what the box does now.

# Examples

```rust
use crawrad_runtime_makepad::welcome::reopen_note;

assert!(reopen_note(true).starts_with("On:"));
assert!(reopen_note(false).starts_with("Off:"));
```
