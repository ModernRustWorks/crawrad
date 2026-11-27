---
id: creation-createdform
kind: doc
target_file: src/creation.rs
line_hint: 531
author: 
created: 
title: "CreatedForm"
tags: []
signature: "pub struct CreatedForm"
---

The newly created form, and what went wrong without preventing creation.

The warning exists because the form and its generated code are two writes, and the
second one can fail on its own — no crate member to generate into, a target that
refuses the document. The form is already on disk by then, so the honest answer is
to keep it and say what did not come with it. Refusing the form over the second
write would discard the first.

# Examples

```rust
use std::path::PathBuf;
use crawrad_project::creation::CreatedForm;

let form = CreatedForm {
    path: PathBuf::from("my_app/src/form_2.crawform"),
    warning: None,
};
assert!(form.warning.is_none());
assert_eq!(form.path.extension().and_then(|e| e.to_str()), Some("crawform"));
```
