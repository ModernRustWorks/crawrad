---
id: ide-app-mcp-host-mcpserver-with-audit-log
kind: doc
target_file: src/ide_app/mcp_host.rs
line_hint: 67
author: 
created: 
title: "McpServer::with_audit_log"
tags: []
signature: "pub fn with_audit_log (mut self , path : PathBuf) -> Self"
---

Sets a custom audit log file (used in tests).

# Examples

```rust,ignore
// binary crate: doctests cannot reach main.rs items
let server = McpServer::new("test").with_audit_log(PathBuf::from("audit.log"));
assert_eq!(server.audit_log_path, Some(PathBuf::from("audit.log")));
```
