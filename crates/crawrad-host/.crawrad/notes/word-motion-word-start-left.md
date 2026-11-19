---
id: word-motion-word-start-left
kind: doc
target_file: src/word_motion.rs
line_hint: 31
author: 
created: 
title: "word_start_left"
tags: []
signature: "pub fn word_start_left (line : & str , byte : usize) -> Option < usize >"
---

Finds the byte index where `Ctrl+Left` stops on the current line.

Where `Ctrl+Left` stops, starting from `byte` on this line. `None` means
there is nothing left on this line — the caller crosses to the end of the
line above, as a modern editor does. Spaces are skipped first, so the start of an
indentation is one step, not one per level.

# Examples

```rust
use crawrad_host::word_motion::word_start_left;

assert_eq!(word_start_left("hello world", 8), Some(6));
assert_eq!(word_start_left("hello", 0), None);
```
