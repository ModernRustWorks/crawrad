---
id: debug-debug-snapshot
kind: doc
target_file: src/debug.rs
line_hint: 307
author: 
created: 
title: "Debug::snapshot"
tags: []
signature: "pub fn snapshot (& self) -> serde_json :: Value"
---

Returns the debug session state snapshot for the MCP.

The session as the MCP reads it (`debug.state`): the phase, where the
program stands, its stack and locals (already in the presenter's
text) and the last lines it printed.

# Examples

```rust
use crawrad_host::Debug;

let debug = Debug::default();
let json = debug.snapshot();
assert_eq!(json["phase"], "idle");
```
