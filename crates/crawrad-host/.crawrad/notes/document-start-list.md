---
id: document-start-list
kind: doc
target_file: src/document.rs
line_hint: 95
author: 
created: 
title: "start_list"
tags: []
signature: "pub fn start_list (arg : Option < String > , var : Option < String > , last : Option < PathBuf > , bases : & [PathBuf] ,) -> Vec < PathBuf >"
---

Determines the candidate forms to open at startup.

The start list (#555): what was named — the command line, then
[`VAR_FORM`], see [`candidatos`] — and, when nothing was, only the `last`
project's form. A named form is an order, and the last project never goes
in front of it.

# Examples

```rust
use std::path::PathBuf;
use crawrad_host::document::start_list;

let list = start_list(None, None, Some(PathBuf::from("last.crawform")), &[]);
assert_eq!(list, vec![PathBuf::from("last.crawform")]);
```
