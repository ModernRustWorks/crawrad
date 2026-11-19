---
id: hostmodel-debug-state-json
kind: doc
target_file: src/lib.rs
line_hint: 257
author: 
created: 
title: "HostModel::debug_state_json"
tags: []
signature: "pub fn debug_state_json (& self) -> serde_json :: Value"
---

The MCP's `debug.state`: the session and every breakpoint.

# Examples

```rust
use crawrad_host::HostModel;

let host = HostModel::new();
let json = host.debug_state_json();
assert_eq!(json["phase"], "idle");
```
