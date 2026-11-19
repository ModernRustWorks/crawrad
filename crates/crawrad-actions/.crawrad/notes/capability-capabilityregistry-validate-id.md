---
id: capability-capabilityregistry-validate-id
kind: doc
target_file: src/capability.rs
line_hint: 506
author: 
created: 
title: "CapabilityRegistry::validate_id"
tags: []
signature: "pub fn validate_id (id : & str) -> Result < () , CapabilityError >"
---

Validates whether an identifier adheres to the `namespace.command` pattern.

Validates whether an identifier adheres to the canonical `namespace.command` pattern.

# Errors

Returns [`CapabilityError::InvalidId`] if the identifier format is invalid.

# Examples

```rust
use crawrad_actions::capability::CapabilityRegistry;

assert!(CapabilityRegistry::validate_id("file.save").is_ok());
assert!(CapabilityRegistry::validate_id("invalid").is_err());
```
