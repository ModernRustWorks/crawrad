---
id: toolset-toolset-parse
kind: doc
target_file: src/toolset.rs
line_hint: 76
author: 
created: 
title: "Toolset::parse"
tags: []
signature: "pub fn parse (spec : & str) -> Result < Self , String >"
---

Parses a comma-separated string of tool group names into a Toolset.

`"design,debug"` → those groups. An unknown name is an error that
lists the valid ones; `"all"` or an empty list means every tool.

# Examples

```rust
use crawrad_mcp::toolset::Toolset;

let ts = Toolset::parse("design,code").unwrap();
assert!(ts.allows("designer.add_component"));
```

# Errors

Returns `Err` with a descriptive error string if an unknown group name is specified.
