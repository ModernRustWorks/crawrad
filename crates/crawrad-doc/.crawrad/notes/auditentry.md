---
id: auditentry
kind: doc
target_file: src/lib.rs
line_hint: 159
author: 
created: 
title: "AuditEntry"
tags: []
signature: "pub struct AuditEntry"
---

Uma entrada do log de auditoria.

# Examples

```rust
use std::collections::BTreeSet;
use crawrad_doc::{AuditEntry, Author};

let entry = AuditEntry {
    revision: 1,
    author: Author::User,
    label: "init".into(),
    touched: BTreeSet::new(),
    undo: false,
};
assert_eq!(entry.revision, 1);
```
