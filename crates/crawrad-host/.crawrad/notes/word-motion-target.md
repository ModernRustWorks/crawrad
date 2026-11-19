---
id: word-motion-target
kind: doc
target_file: src/word_motion.rs
line_hint: 93
author: 
created: 
title: "target"
tags: []
signature: "pub fn target (lines : & [String] , (l , b) : (usize , usize) , m : Motion) -> (usize , usize)"
---

Calculates the cursor landing position `(line, byte)` for a motion.

Where the cursor lands, as `(line, byte)` from zero, on `lines` — the
editor's own text, not a copy that could be one keystroke behind.

# Examples

```rust
use crawrad_host::word_motion::{target, Motion};

let lines = vec!["hello world".to_string()];
assert_eq!(target(&lines, (0, 8), Motion::WordLeft), (0, 6));
```
