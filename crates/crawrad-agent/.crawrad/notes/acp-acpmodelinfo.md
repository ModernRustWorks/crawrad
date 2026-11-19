---
id: acp-acpmodelinfo
kind: doc
target_file: src/acp.rs
line_hint: 16
author: 
created: 
title: "AcpModelInfo"
tags: []
signature: "pub type AcpModelInfo = ModelSpec"
---

Descriptive metadata of a language model supported by ACP.

# Examples

```rust
use crawrad_agent::AcpModelInfo;

let model = AcpModelInfo {
    id: "sonnet".to_string(),
    display_name: "Claude Sonnet".to_string(),
    description: "Balanced model".to_string(),
    supports_reasoning: false,
};
assert_eq!(model.id, "sonnet");
```
