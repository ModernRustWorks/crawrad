---
id: targets-data-binding-open-datasets-code
kind: doc
target_file: src/targets/data_binding.rs
line_hint: 247
author: 
created: 
title: "open_datasets_code"
tags: []
signature: "pub fn open_datasets_code (datasets : & [(String , String)] , form : & str) -> String"
---

Opens every bound dataset once, on the first frame/sync, from the form's fields.

# Examples

```rust
use crawrad_codegen::targets::data_binding::open_datasets_code;

let code = open_datasets_code(&[], "self.");
assert!(code.is_empty());
```

`form` is how the target reaches the form's fields: `self.` (egui) or `self.form.`
(Makepad); the adapter and its flags always live on `self`.
