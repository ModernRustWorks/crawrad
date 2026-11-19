---
id: acp-registry-cache-timestamp
kind: doc
target_file: src/acp_registry.rs
line_hint: 475
author: 
created: 
title: "cache_timestamp"
tags: []
signature: "pub fn cache_timestamp () -> Option < String >"
---

Returns the last-modified timestamp of the local cache file, if it exists.

# Examples

```rust
use crawrad_agent::acp_registry::cache_timestamp;

if let Some(ts) = cache_timestamp() {
    assert!(!ts.is_empty());
}
```
