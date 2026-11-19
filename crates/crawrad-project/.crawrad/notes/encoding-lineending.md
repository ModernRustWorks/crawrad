---
id: encoding-lineending
kind: doc
target_file: src/encoding.rs
line_hint: 28
author: 
created: 
title: "LineEnding"
tags: []
signature: "pub enum LineEnding"
---

What the file used to end a line.

It is remembered because the text an editor edits is **always** `\n`: a
stray `\r` in the middle draws as a missing-glyph box and throws the cursor
count off. Normalise on open, undo on save.

# Examples

```rust
use crawrad_project::encoding::LineEnding;

assert_eq!(LineEnding::default(), LineEnding::Lf);
```
