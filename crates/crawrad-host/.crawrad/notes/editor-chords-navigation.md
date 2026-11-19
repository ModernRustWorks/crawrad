---
id: editor-chords-navigation
kind: doc
target_file: src/editor_chords.rs
line_hint: 26
author: 
created: 
title: "Navigation"
tags: []
signature: "pub struct Navigation"
---

A navigation key, already read.

# Examples

```rust
use crawrad_host::editor_chords::{Motion, Navigation};

let nav = Navigation { motion: Motion::WordLeft, extend: false };
assert_eq!(nav.motion, Motion::WordLeft);
```
