---
id: acp-registry-registry-browse-url
kind: doc
target_file: src/acp_registry.rs
line_hint: 639
author: 
created: 
title: "registry_browse_url"
tags: []
signature: "pub fn registry_browse_url (query : & str) -> String"
---

Builds the URL to open in the browser for the registry link.

# Examples

```rust
use crawrad_agent::acp_registry::registry_browse_url;

let url = registry_browse_url("claude");
assert!(url.contains("claude"));
```
