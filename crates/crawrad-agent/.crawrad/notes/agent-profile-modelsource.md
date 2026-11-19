---
id: agent-profile-modelsource
kind: doc
target_file: src/agent_profile.rs
line_hint: 55
author: 
created: 
title: "ModelSource"
tags: []
signature: "pub enum ModelSource"
---

Strategy for obtaining available models for an agent profile.

# Examples

```rust
use crawrad_agent::ModelSource;

let source = ModelSource::Static(&[]);
assert!(matches!(source, ModelSource::Static(_)));
```
