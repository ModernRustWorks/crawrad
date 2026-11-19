---
id: rename-rename
kind: doc
target_file: src/rename.rs
line_hint: 135
author: 
created: 
title: "Rename"
tags: []
signature: "pub struct Rename"
---

Rename symbol dialog state tracking workflow stage and user input.

The dialog's state: the symbol, where it was asked, the new name, the
stage, and a sentence when something is refused or fails.

# Examples

```rust
use crawrad_host::rename::Rename;

let rename = Rename::default();
assert!(!rename.is_open());
```
