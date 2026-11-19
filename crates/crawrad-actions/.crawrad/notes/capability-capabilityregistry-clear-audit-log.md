---
id: capability-capabilityregistry-clear-audit-log
kind: doc
target_file: src/capability.rs
line_hint: 918
author: 
created: 
title: "CapabilityRegistry::clear_audit_log"
tags: []
signature: "pub fn clear_audit_log (& mut self)"
---

Clears the audit history.

# Examples

```rust
use crawrad_actions::capability::CapabilityRegistry;

let mut reg = CapabilityRegistry::new();
reg.clear_audit_log();
assert!(reg.audit_log().is_empty());
```
