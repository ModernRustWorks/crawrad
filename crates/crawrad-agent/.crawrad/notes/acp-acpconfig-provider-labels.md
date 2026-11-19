---
id: acp-acpconfig-provider-labels
kind: doc
target_file: src/acp.rs
line_hint: 271
author: 
created: 
title: "AcpConfig::provider_labels"
tags: []
signature: "pub fn provider_labels () -> Vec < String >"
---

Returns the provider labels formatted for dropdowns.

# Examples

```rust
use crawrad_agent::AcpConfig;

let labels = AcpConfig::provider_labels();
assert!(!labels.is_empty());
```
