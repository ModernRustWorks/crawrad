---
id: audit-auditentry
kind: doc
target_file: src/audit.rs
line_hint: 13
author: 
created: 
title: "AuditEntry"
tags: []
signature: "pub struct AuditEntry"
---

A single audit record.

# Examples

```rust
use crawrad_mcp::AuditEntry;

let entry = AuditEntry {
    ts: "123".into(),
    tool: "code.read".into(),
    args: serde_json::json!({}),
    outcome: "applied".into(),
    session_id: "s1".into(),
};
assert_eq!(entry.tool, "code.read");
```
