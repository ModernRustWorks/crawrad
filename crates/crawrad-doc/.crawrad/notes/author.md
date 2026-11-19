---
id: author
kind: doc
target_file: src/lib.rs
line_hint: 75
author: 
created: 
title: "Author"
tags: []
signature: "pub enum Author"
---

Identifies the author of a document modification.

Who requested the change. Goes to auditing and allows answering "who touched
this?" — a question that no classic IDE answers well.

# Examples

```rust
use crawrad_doc::Author;

let author = Author::User;
assert_eq!(author.to_string(), "user");
```
