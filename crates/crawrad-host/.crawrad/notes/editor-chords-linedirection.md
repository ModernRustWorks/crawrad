---
id: editor-chords-linedirection
kind: doc
target_file: src/editor_chords.rs
line_hint: 34
author: 
created: 
title: "LineDirection"
tags: []
signature: "pub enum LineDirection"
---

Which way lines move or duplicate.

# Examples

```rust
use crawrad_host::editor_chords::LineDirection;

let dir = LineDirection::Up;
assert_eq!(dir, LineDirection::Up);
```
