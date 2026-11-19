---
id: capability-capability-custom-executor
kind: doc
target_file: src/capability.rs
line_hint: 312
author: 
created: 
title: "Capability::custom_executor"
tags: []
signature: "pub fn custom_executor < E > (id : impl Into < String > , label : impl Into < String > , description : impl Into < String > , group : impl Into < String > , source : CapabilitySource , executor : E ,) -> Self where E : CapabilityExecutor + 'static ,"
---

Creates a new custom capability with a structured executor.

# Examples

```rust
use crawrad_actions::capability::{Capability, CapabilityError, CapabilityExecutor, CapabilitySource};
use crawrad_actions::{Context, Effect};

struct MyExec;
impl CapabilityExecutor for MyExec {
    fn execute(&self, _ctx: &Context<'_>) -> Result<Effect, CapabilityError> {
        Ok(Effect::none())
    }
}

let cap = Capability::custom_executor("my.tool", "My Tool", "Executes tool", "Tools", CapabilitySource::Native, MyExec);
assert_eq!(cap.id, "my.tool");
```
