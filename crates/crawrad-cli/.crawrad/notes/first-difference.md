---
id: first-difference
kind: doc
target_file: src/lib.rs
line_hint: 120
author: 
created: 
title: "first_difference"
tags: []
signature: "pub fn first_difference (found : & str , expected : & str) -> Option < String >"
---

The first line where two texts disagree, ready for the screen.

Line breaks are the repository's business, not the manifest's: this tree
arrives with CRLF on Windows and LF elsewhere, and a gate that failed over
that would be measuring the checkout instead of the content.

# Examples

```rust
use crawrad_cli::first_difference;

assert_eq!(first_difference("a\n", "a\n"), None);
assert!(first_difference("a\n", "b\n").is_some());
```
