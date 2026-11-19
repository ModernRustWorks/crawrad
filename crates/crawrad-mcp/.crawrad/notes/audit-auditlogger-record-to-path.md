---
id: audit-auditlogger-record-to-path
kind: doc
target_file: src/audit.rs
line_hint: 51
author: 
created: 
title: "AuditLogger::record_to_path"
tags: []
signature: "pub fn record_to_path (path : & Path , tool : & str , args : & Value , outcome : & str , session_id : & str)"
---

Records an audit log entry to a specific file path (useful for testing).

# Examples

```rust
use crawrad_mcp::AuditLogger;

let dir = std::env::temp_dir().join("test_audit.jsonl");
AuditLogger::record_to_path(&dir, "code.read", &serde_json::json!({}), "applied", "sess1");
```
