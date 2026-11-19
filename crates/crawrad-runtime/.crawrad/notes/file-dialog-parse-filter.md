---
id: file-dialog-parse-filter
kind: doc
target_file: src/file_dialog.rs
line_hint: 36
author: 
created: 
title: "parse_filter"
tags: []
signature: "pub fn parse_filter (classic : & str) -> Vec < FileFilter >"
---

Parses a classic filter string into structured file filters.

Parses the classic filter string (`"Description|*.ext;*.ext2|..."`) into structured [`FileFilter`]s.

Unpaired trailing filter elements (odd count) and empty inputs are ignored safely.

# Examples

```rust
use crawrad_runtime::file_dialog::parse_filter;

let filters = parse_filter("Text files (*.txt)|*.txt|All files (*.*)|*.*");
assert_eq!(filters.len(), 2);
assert_eq!(filters[0].name, "Text files (*.txt)");
```
