---
id: ide-app-chrome-ideapp-rail-json
kind: doc
target_file: src/ide_app/chrome.rs
line_hint: 613
author: 
created: 
title: "IdeApp::rail_json"
tags: []
signature: "pub (crate) fn rail_json (& self) -> serde_json :: Value"
---

Serializes the Activity Rail panels and rects to JSON.

Returns `{ "panels": [ { "panel": <label>, "rect": [x, y, w, h] } ] }`.

# Examples

```rust
let json = app.rail_json();
assert!(json.get("panels").is_some());
```
