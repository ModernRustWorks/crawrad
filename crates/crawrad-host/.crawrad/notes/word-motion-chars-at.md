---
id: word-motion-chars-at
kind: doc
target_file: src/word_motion.rs
line_hint: 24
author: 
created: 
title: "chars_at"
tags: []
signature: "fn chars_at (line : & str , byte : usize) -> (Vec < (usize , char) > , usize)"
---

The characters of a line, as `(byte, char)`, plus where `byte` falls in it.

A byte that lands INSIDE a character rounds up to the next boundary:
slicing a `&str` there panics, and a frame where the caret and a copy of
the text disagree must not take the IDE down.
