---
id: capability-capabilityexecutor
kind: doc
target_file: src/capability.rs
line_hint: 138
author: 
created: 
title: "CapabilityExecutor"
tags: []
signature: "pub trait CapabilityExecutor : Send + Sync"
---

Abstract executor for capability dispatch.

# Examples

```rust
use crawrad_actions::capability::{CapabilityError, CapabilityExecutor};
use crawrad_actions::{Context, Effect};

struct MyExecutor;

impl CapabilityExecutor for MyExecutor {
    fn execute(&self, _ctx: &Context<'_>) -> Result<Effect, CapabilityError> {
        Ok(Effect::none())
    }
}

let exec = MyExecutor;
```
