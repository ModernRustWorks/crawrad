---
id: capability-capabilityregistry-audit-log
kind: doc
target_file: src/capability.rs
line_hint: 913
author: 
created: 
title: "CapabilityRegistry::audit_log"
tags: []
signature: "pub fn audit_log (& self) -> & [AuditEntry]"
---

Returns the audit log entries of executions.

# Examples

```rust
use crawrad_actions::capability::CapabilityRegistry;

let reg = CapabilityRegistry::new();
assert!(reg.audit_log().is_empty());
```
