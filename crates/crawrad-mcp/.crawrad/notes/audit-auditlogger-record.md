---
id: audit-auditlogger-record
kind: doc
target_file: src/audit.rs
line_hint: 46
author: 
created: 
title: "AuditLogger::record"
tags: []
signature: "pub fn record (tool : & str , args : & Value , outcome : & str , session_id : & str)"
---

Records an audit log entry to the standard path.

Records an audit log entry to the standard path. Never panics or returns an error.

# Examples

```rust
use crawrad_mcp::AuditLogger;

AuditLogger::record("code.read", &serde_json::json!({}), "applied", "sess1");
```
