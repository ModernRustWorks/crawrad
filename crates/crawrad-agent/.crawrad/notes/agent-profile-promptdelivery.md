---
id: agent-profile-promptdelivery
kind: doc
target_file: src/agent_profile.rs
line_hint: 30
author: 
created: 
title: "PromptDelivery"
tags: []
signature: "pub enum PromptDelivery"
---

Prompt delivery mechanism for the agent CLI invocation.

# Examples

```rust
use crawrad_agent::PromptDelivery;

let p = PromptDelivery::Stdin;
assert_eq!(p, PromptDelivery::Stdin);
```
