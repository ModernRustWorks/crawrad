---
id: acp-registry-registry-link-text
kind: doc
target_file: src/acp_registry.rs
line_hint: 664
author: 
created: 
title: "registry_link_text"
tags: []
signature: "pub fn registry_link_text (query : & str) -> String"
---

Builds the link text shown below the search box.

# Examples

```rust
use crawrad_agent::acp_registry::registry_link_text;

let text = registry_link_text("");
assert!(text.contains("Browse all"));
```
