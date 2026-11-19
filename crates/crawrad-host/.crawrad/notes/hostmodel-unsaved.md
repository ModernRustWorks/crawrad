---
id: hostmodel-unsaved
kind: doc
target_file: src/lib.rs
line_hint: 350
author: 
created: 
title: "HostModel::unsaved"
tags: []
signature: "pub fn unsaved (& self , editable : Option < & Path > , saved_revision : u64) -> (bool , bool)"
---

Returns whether Save and Save All have unsaved changes to write.

What (Save, Save All) have to write: Save, the `editable` file's
unsaved edit or the form changed since `saved_revision`; Save All, any
unsaved edit or the form. Both `false` lets the toolbar dim them.

# Examples

```rust
use crawrad_host::HostModel;

let host = HostModel::new();
assert_eq!(host.unsaved(None, 0), (false, false));
```
