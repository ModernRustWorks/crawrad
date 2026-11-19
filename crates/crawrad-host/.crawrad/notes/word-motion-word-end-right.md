---
id: word-motion-word-end-right
kind: doc
target_file: src/word_motion.rs
line_hint: 50
author: 
created: 
title: "word_end_right"
tags: []
signature: "pub fn word_end_right (line : & str , byte : usize) -> Option < usize >"
---

Finds the byte index where `Ctrl+Right` stops on the current line.

Where `Ctrl+Right` stops, starting from `byte` on this line. `None` means
there is nothing right on this line — the caller crosses to the start of
the line below.

# Examples

```rust
use crawrad_host::word_motion::word_end_right;

assert_eq!(word_end_right("hello world", 0), Some(5));
assert_eq!(word_end_right("hello", 5), None);
```
