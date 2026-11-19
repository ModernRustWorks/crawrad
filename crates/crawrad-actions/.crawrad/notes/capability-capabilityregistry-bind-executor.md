---
id: capability-capabilityregistry-bind-executor
kind: doc
target_file: src/capability.rs
line_hint: 646
author: 
created: 
title: "CapabilityRegistry::bind_executor"
tags: []
signature: "pub fn bind_executor < E > (& mut self , id : & str , executor : E) -> Result < () , CapabilityError > where E : CapabilityExecutor + 'static ,"
---

Associates a concrete executor with a previously registered capability.

# Errors

Returns [`CapabilityError::NotFound`] if no capability exists with the given ID.

# Examples

```rust
use crawrad_actions::capability::{Capability, CapabilityError, CapabilityExecutor, CapabilityRegistry, CapabilitySource};
use crawrad_actions::{Context, Effect};

struct MyExec;
impl CapabilityExecutor for MyExec {
    fn execute(&self, _ctx: &Context<'_>) -> Result<Effect, CapabilityError> {
        Ok(Effect::none())
    }
}

let mut reg = CapabilityRegistry::new();
let cap = Capability::custom("custom.test", "Test", "Desc", "Group", CapabilitySource::Native, |_| Ok(Effect::none()));
reg.register(cap).unwrap();
assert!(reg.bind_executor("custom.test", MyExec).is_ok());
```
