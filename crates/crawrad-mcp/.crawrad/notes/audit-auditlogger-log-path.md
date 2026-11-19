---
id: audit-auditlogger-log-path
kind: doc
target_file: src/audit.rs
line_hint: 31
author: 
created: 
title: "AuditLogger::log_path"
tags: []
signature: "pub fn log_path () -> PathBuf"
---

Returns the standard audit log path.

# Examples

```rust
use crawrad_mcp::AuditLogger;

let p = AuditLogger::log_path();
assert!(p.to_string_lossy().contains("mcp_audit.jsonl"));
```
