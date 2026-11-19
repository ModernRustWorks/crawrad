---
id: semantic-legend-from-capabilities
kind: doc
target_file: src/semantic.rs
line_hint: 50
author: 
created: 
title: "Legend::from_capabilities"
tags: []
signature: "pub fn from_capabilities (capabilities : Option < & Value >) -> Self"
---

`semanticTokensProvider.legend` of the server's capabilities.

# Examples

```rust
use crawrad_lsp::semantic::Legend;
use serde_json::json;

let caps = json!({
    "semanticTokensProvider": {
        "legend": {
            "tokenTypes": ["macro", "function"],
            "tokenModifiers": ["deprecated"]
        }
    }
});
let legend = Legend::from_capabilities(Some(&caps));
assert_eq!(legend.types.len(), 2);
```
