---
id: edit-wrong-view
kind: doc
target_file: src/edit.rs
line_hint: 127
author: 
created: 
title: "wrong_view"
tags: []
signature: "pub fn wrong_view (needs : crawrad_actions :: ViewAffinity) -> (& 'static str , & 'static str)"
---

Refuses a tool call when the active view does not match the affinity.

The refusal of a tool sent while the other view is in front (rule #1):
its reason code and the sentence that says how to switch.

# Examples

```rust
use crawrad_mcp::edit::wrong_view;
use crawrad_actions::ViewAffinity;

let (code, msg) = wrong_view(ViewAffinity::Design);
assert_eq!(code, "wrong-view-code");
assert!(msg.contains("form designer is not active"));
```
