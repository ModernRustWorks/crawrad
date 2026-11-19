---
id: note-brokenlink
kind: doc
target_file: src/note.rs
line_hint: 72
author: 
created: 
title: "BrokenLink"
tags: []
signature: "pub struct BrokenLink"
---

An intra-doc link that could not be resolved or points to a private item.

An intra-doc link that rustdoc could not resolve or that points to a private item.

# Examples

```rust
use crawrad_cli::note::BrokenLink;

let link = BrokenLink {
    file: "src/lib.rs".to_string(),
    line: 42,
    note: Some("doc_id".to_string()),
    message: "unresolved link to `Foo`".to_string(),
};
assert_eq!(link.line, 42);
```
