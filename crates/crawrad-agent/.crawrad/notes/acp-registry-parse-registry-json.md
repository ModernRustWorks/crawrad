---
id: acp-registry-parse-registry-json
kind: doc
target_file: src/acp_registry.rs
line_hint: 836
author: 
created: 
title: "parse_registry_json"
tags: []
signature: "pub fn parse_registry_json (json : & str) -> Result < Vec < AcpAgentEntry > , String >"
---

Parses a raw registry JSON string into agent entries.

Exposed for testing with fixture data.

# Errors

Returns an error if the JSON is invalid or fails to deserialize.

# Examples

```rust
use crawrad_agent::acp_registry::parse_registry_json;

let json = r#"{"version": "1.0", "agents": []}"#;
let agents = parse_registry_json(json).unwrap();
assert!(agents.is_empty());
```
