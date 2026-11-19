---
id: explorer-rows-rows-form-form
kind: doc
target_file: src/explorer_rows/rows/form.rs
line_hint: 15
author: 
created: 
title: "Form"
tags: []
signature: "pub struct Form"
---

One form of a member.

# Examples

```rust
use std::path::Path;
use crawrad_project::explorer_rows::rows::form::Form;
use crawrad_project::explorer_rows::row::ExplorerRow;

let form = Form::new(Path::new("forms/Main.crawform"), "Main", true);
assert_eq!(form.text(), "Main");
```
