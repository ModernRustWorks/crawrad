---
id: link-pick-linkpick-to-json
kind: doc
target_file: src/link_pick.rs
line_hint: 128
author: 
created: 
title: "LinkPick::to_json"
tags: []
signature: "pub fn to_json (& self) -> serde_json :: Value"
---

What MCP reads (`editor.get_choices`): open, and the rows' labels.

# Examples

```rust
use crawrad_host::link_pick::LinkPick;

let pick = LinkPick::default();
let json = pick.to_json();
assert_eq!(json["open"], false);
```
