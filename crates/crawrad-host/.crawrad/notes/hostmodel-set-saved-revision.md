---
id: hostmodel-set-saved-revision
kind: doc
target_file: src/lib.rs
line_hint: 332
author: 
created: 
title: "HostModel::set_saved_revision"
tags: []
signature: "pub fn set_saved_revision (& mut self, revision : u64)"
---

Records the revision the form in front was just saved at.

With nothing in front this does nothing, rather than storing a revision that
belongs to no form.

# Examples

```rust
use crawrad_host::HostModel;
use crawrad_doc::Doc;
use crawrad_ir::Document;

let mut host = HostModel::new();
host.open_form(Doc::new(Document::new("form_main", "Form")), None);
host.set_saved_revision(7);
assert_eq!(host.saved_revision(), 7);
```
