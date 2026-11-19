---
id: effect
kind: doc
target_file: src/lib.rs
line_hint: 565
author: 
created: 
title: "Effect"
tags: []
signature: "pub struct Effect"
---

Declarative description of what an action wants to happen.

What the action wants to happen. Declarative on purpose: the action applies
nothing, so it can be executed both by the IDE and by an agent.

# Examples

```rust
use crawrad_actions::Effect;

let effect = Effect::default();
assert!(effect.transaction.is_none());
```
