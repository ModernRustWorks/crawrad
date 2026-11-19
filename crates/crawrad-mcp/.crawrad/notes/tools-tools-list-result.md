---
id: tools-tools-list-result
kind: doc
target_file: src/tools.rs
line_hint: 1068
author: 
created: 
title: "tools_list_result"
tags: []
signature: "pub fn tools_list_result (registry : Option < & CapabilityRegistry >) -> serde_json :: Value"
---

Builds the tools/list JSON result with catalogue tools and extension names.

The `tools/list` result: the tools, and in `_meta` the catalogue version
and `extensionTools`, the names extensions contributed (#339) — the bridge
lets them through only in its `extensions` group, whatever their prefix.

# Examples

```rust
use crawrad_mcp::tools::tools_list_result;

let res = tools_list_result(None);
assert!(res.get("tools").is_some());
```
