---
id: note-preview-doctestrun
kind: doc
target_file: src/note_preview.rs
line_hint: 129
author: 
created: 
title: "DoctestRun"
tags: []
signature: "pub struct DoctestRun"
---

One run of an item's doctests.

# The filter is the item path, and the reading picks the item

rustdoc names a doctest `<file> - <path> (line N)`, and the harness takes
the filter as a substring, so `Client::fetch` also runs `Client::fetch_all`.
A filter with a space cannot narrow it: rustdoc splits the test arguments
on whitespace, and `Client::fetch (line` became `Client::fetch` plus
`(line`, which matches every doctest (measured on a real crate). So the
run filters by the path and [`parse_doctest_output`] counts only the lines
of exactly this path.

# Examples

```rust
use crawrad_project::note_preview::DoctestRun;

let run = DoctestRun::new("my_crate", Some("MyStruct"), "my_fn");
assert_eq!(run.path, "MyStruct::my_fn");
```
