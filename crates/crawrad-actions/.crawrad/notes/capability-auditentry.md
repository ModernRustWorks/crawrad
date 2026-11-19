---
id: capability-auditentry
kind: doc
target_file: src/capability.rs
line_hint: 415
author: 
created: 
title: "AuditEntry"
tags: []
signature: "pub struct AuditEntry"
---

Audit record generated upon each capability dispatch.

# Examples

```rust
use crawrad_actions::capability::AuditEntry;
use crawrad_doc::Author;

let entry = AuditEntry {
    id: 1,
    timestamp_ms: 1000,
    capability_id: "test.cmd".to_string(),
    author: Author::User,
    success: true,
    error: None,
    effect_summary: "ok".to_string(),
};
assert_eq!(entry.id, 1);
```
