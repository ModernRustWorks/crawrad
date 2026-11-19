---
id: document-candidatos-2
kind: doc
target_file: src/document.rs
line_hint: 46
author: 
created: 
title: "candidatos"
tags: []
signature: "pub fn candidatos (arg : Option < String > , var : Option < String > , bases : & [PathBuf]) -> Vec < PathBuf >"
---

Assembles the ordered list of candidate paths to open the form.

The search order: the command-line argument, then [`VAR_FORM`],
then the repository's [`CANDIDATOS`] — each relative to the working
directory and to the repository root.

An absolute path enters only once: joining a base to it changes nothing, and the
attempt list in the error message would be duplicated.

# Examples

```rust
use std::path::PathBuf;
use crawrad_host::document::candidatos;

let list = candidatos(Some("test.crawform".into()), None, &[PathBuf::from(".")]);
assert_eq!(list, vec![PathBuf::from("./test.crawform")]);
```
