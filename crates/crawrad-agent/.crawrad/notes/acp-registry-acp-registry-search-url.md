---
id: acp-registry-acp-registry-search-url
kind: doc
target_file: src/acp_registry.rs
line_hint: 22
author: 
created: 
title: "ACP_REGISTRY_SEARCH_URL"
tags: []
signature: "pub const ACP_REGISTRY_SEARCH_URL : & str = \"https://github.com/anthropics/agent-client-protocol/blob/main/registry/registry.json\""
---

GitHub search URL for the registry repo.

# Examples

```rust
use crawrad_agent::acp_registry::ACP_REGISTRY_SEARCH_URL;

assert!(ACP_REGISTRY_SEARCH_URL.contains("github.com"));
```
