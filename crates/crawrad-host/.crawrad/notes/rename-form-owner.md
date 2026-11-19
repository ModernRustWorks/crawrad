---
id: rename-form-owner
kind: doc
target_file: src/rename.rs
line_hint: 57
author: 
created: 
title: "form_owner"
tags: []
signature: "pub fn form_owner (name : & str , forms : & [(String , & crawrad_ir :: Document)]) -> Option < Owner >"
---

Checks whether an identifier is owned by any project form.

Does a form own `name`? `forms` are each form's name and document — the
open one as it is in memory, the others from the disk.

# Examples

```rust
use crawrad_host::rename::form_owner;

assert!(form_owner("btn_ok", &[]).is_none());
```
