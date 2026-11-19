---
id: palette-to-json
kind: doc
target_file: src/palette.rs
line_hint: 140
author: 
created: 
title: "to_json"
tags: []
signature: "pub fn to_json (rows : & [Row], filter : & str) -> serde_json :: Value"
---
The palette as the MCP `view.get_toolbox` reads it.

# Examples

```
use crawrad_designer::palette::{to_json, Row};
let v = to_json(&[Row::Empty], "zz");
assert_eq!(v["filter"], "zz");
```
