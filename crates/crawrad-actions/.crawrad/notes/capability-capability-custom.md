---
id: capability-capability-custom
kind: doc
target_file: src/capability.rs
line_hint: 284
author: 
created: 
title: "Capability::custom"
tags: []
signature: "pub fn custom < F > (id : impl Into < String > , label : impl Into < String > , description : impl Into < String > , group : impl Into < String > , source : CapabilitySource , executor : F ,) -> Self where F : Fn (& Context < '_ >) -> Result < Effect , CapabilityError > + Send + Sync + 'static ,"
---

Creates a new custom capability with an executor closure.

# Examples

```rust
use crawrad_actions::capability::{Capability, CapabilitySource};
use crawrad_actions::Effect;

let cap = Capability::custom("ext.action", "Action", "Description", "Group", CapabilitySource::Native, |_| {
    Ok(Effect::none())
});
assert_eq!(cap.label, "Action");
```
