---
id: server-says-watch-heard
kind: doc
target_file: src/server_says.rs
line_hint: 92
author:
created:
title: "heard"
tags: []
signature: "pub fn heard (& self , method : & str , params : & serde_json :: Value)"
---

Reads one server notification and updates the verdict.

Anything that is not one of the notifications we listen for leaves the
verdict alone — silence is not news.

# Examples

```rust
use crawrad_lsp::server_says::{Health, Watch};
use serde_json::json;

let watch = Watch::new();
watch.heard(
    "window/showMessage",
    &json!({ "type": 1, "message": "failed to load manifest for x" }),
);
assert!(watch.health().cannot_trust_empty());

// Something it does not listen for leaves the verdict as it was.
watch.heard("telemetry/event", &json!({ "message": "hello" }));
assert!(watch.health().cannot_trust_empty());
```
