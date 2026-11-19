---
id: teacher-book-chapter
kind: doc
target_file: src/teacher.rs
line_hint: 18
author: 
created: 
title: "book_chapter"
tags: []
signature: "pub fn book_chapter (code : & str) -> Option < (& 'static str , & 'static str) >"
---

Returns (title, URL) from the Rust Book for recognized rustc error codes.

Returns (title, URL) from a static table to *The Rust Programming Language* book
for recognized rustc error codes.

# Examples

```rust
use crawrad_agent::teacher::book_chapter;

let chapter = book_chapter("E0382");
assert!(chapter.is_some());
assert_eq!(chapter.unwrap().0, "Understanding Ownership");
```
