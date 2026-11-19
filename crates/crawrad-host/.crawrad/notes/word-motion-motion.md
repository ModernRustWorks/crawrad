---
id: word-motion-motion
kind: doc
target_file: src/word_motion.rs
line_hint: 81
author: 
created: 
title: "Motion"
tags: []
signature: "pub enum Motion"
---

Editor cursor navigation motions handled internally.

A move the editor makes itself (the framework does not): `Ctrl+Left`,
`Ctrl+Right`, `Ctrl+Home`, `Ctrl+End`.

# Examples

```rust
use crawrad_host::word_motion::Motion;

let motion = Motion::WordLeft;
assert_eq!(motion, Motion::WordLeft);
```
