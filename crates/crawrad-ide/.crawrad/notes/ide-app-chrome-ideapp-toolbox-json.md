---
id: ide-app-chrome-ideapp-toolbox-json
kind: doc
target_file: src/ide_app/chrome.rs
line_hint: 628
author: 
created: 
title: "IdeApp::toolbox_json"
tags: []
signature: "pub (crate) fn toolbox_json (& self, cx: & Cx) -> serde_json :: Value"
---

Serializes the Toolbox side panel empty state to JSON.

Returns `{ "empty_state": null }` or `{ "empty_state": { "title", ... } }`.

# Examples

```rust
let json = app.toolbox_json(cx);
assert!(json.get("empty_state").is_some());
```
