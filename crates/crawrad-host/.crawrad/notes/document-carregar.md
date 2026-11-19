---
id: document-carregar
kind: doc
target_file: src/document.rs
line_hint: 69
author: 
created: 
title: "carregar"
tags: []
signature: "pub fn carregar < L > (lista : & [PathBuf] , ler : L) -> Result < Aberto , String > where L : Fn (& Path) -> Result < String , String > ,"
---

Tries the list in order and returns the FIRST that loads.

When none loads, the failure brings **all** attempts with the error of
each. No emergency form is born here: in an IDE, inventing a
document silently is worse than not opening anything.

# Examples

```rust
use std::path::PathBuf;
use crawrad_host::document::carregar;

let res = carregar(&[PathBuf::from("missing.crawform")], |_| Err("not found".into()));
assert!(res.is_err());
```

# Errors

Returns `Err` if no path in the list can be read or parsed as valid JSON.
