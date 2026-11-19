---
id: capability-capabilityenabledcheck
kind: doc
target_file: src/capability.rs
line_hint: 143
author: 
created: 
title: "CapabilityEnabledCheck"
tags: []
signature: "pub trait CapabilityEnabledCheck : Send + Sync"
---

Abstract predicate to check if a capability is enabled.

# Examples

```rust
use crawrad_actions::capability::CapabilityEnabledCheck;
use crawrad_actions::Context;

struct AlwaysEnabled;

impl CapabilityEnabledCheck for AlwaysEnabled {
    fn is_enabled(&self, _ctx: &Context<'_>) -> bool {
        true
    }
}
```
