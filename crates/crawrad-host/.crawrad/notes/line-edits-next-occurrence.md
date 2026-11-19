---
id: line-edits-next-occurrence
kind: doc
target_file: src/line_edits.rs
line_hint: 109
author: 
created: 
title: "next_occurrence"
tags: []
signature: "pub fn next_occurrence (lines : & [String] , term : & str , after : (usize , usize) , before : (usize , usize) ,) -> Option < (usize , usize) >"
---

Finds the next occurrence of a search term with circular wrapping.

The next place `term` appears after `after` (line, byte), wrapping round to
the start and stopping before `before` — `Ctrl+D`'s next occurrence.

# Examples

```rust
use crawrad_host::line_edits::next_occurrence;

let lines = vec!["foo bar".into(), "baz foo".into()];
let found = next_occurrence(&lines, "foo", (0, 3), (0, 0));
assert_eq!(found, Some((1, 4)));
```
