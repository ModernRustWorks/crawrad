---
id: toolset-toolset-filter-list-response
kind: doc
target_file: src/toolset.rs
line_hint: 134
author: 
created: 
title: "Toolset::filter_list_response"
tags: []
signature: "pub fn filter_list_response (& mut self , response : & mut Value)"
---

Drops tools outside this toolset from a tools/list response.

Drops the tools outside this toolset from a `tools/list` response,
first learning from its `_meta.extensionTools` which came from extensions.

# Examples

```rust
use crawrad_mcp::toolset::Toolset;
use serde_json::json;

let mut ts = Toolset::parse("code").unwrap();
let mut resp = json!({
    "result": {
        "tools": [{ "name": "editor.save" }, { "name": "designer.add" }]
    }
});
ts.filter_list_response(&mut resp);
assert_eq!(resp["result"]["tools"].as_array().unwrap().len(), 1);
```
