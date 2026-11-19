---
id: doc-note-depends-on-crawrad-note
kind: doc
target_file: src/doc_note.rs
line_hint: 950
author: 
created: 
title: "depends_on_crawrad_note"
tags: []
signature: "pub fn depends_on_crawrad_note (cargo_toml : & str) -> bool"
---

Checks whether a Cargo.toml manifest depends on `crawrad-note`.

Whether that Cargo.toml lists `crawrad-note` under dependencies (plain key, `.workspace = true`, or table form).

# Examples

```rust
use crawrad_project::doc_note::depends_on_crawrad_note;

assert!(depends_on_crawrad_note("[dependencies]\ncrawrad-note = \"0.1\""));
assert!(!depends_on_crawrad_note("[dependencies]\nserde = \"1.0\""));
```
