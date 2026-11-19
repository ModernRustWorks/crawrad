---
id: word-motion-column-of-byte
kind: doc
target_file: src/word_motion.rs
line_hint: 69
author: 
created: 
title: "column_of_byte"
tags: []
signature: "pub fn column_of_byte (line : & str , byte : usize) -> usize"
---

Computes the 1-based character column of a byte offset inside a line.

**The column of a byte inside a line, counted in CHARACTERS and from 1.**

From 1 because that is what every editor shows, and in characters because
bytes are not columns: `//! Este arquivo é seu` — the first line of every
unit this IDE generates — puts the two out of step at the `é`, and the
footer would claim a column the cursor is not in.

A byte index past the end of the line, or one that lands inside a character,
answers with the nearest boundary instead of panicking: the cursor comes
from the editor and the text from our own last write, and a frame where the
two disagree must not take the IDE down.

# Examples

```rust
use crawrad_host::word_motion::column_of_byte;

assert_eq!(column_of_byte("hello", 2), 3);
assert_eq!(column_of_byte("café", 5), 5);
```
