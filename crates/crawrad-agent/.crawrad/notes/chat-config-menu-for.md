---
id: chat-config-menu-for
kind: doc
target_file: src/chat_config.rs
line_hint: 42
author: 
created: 
title: "menu_for"
tags: []
signature: "pub fn menu_for (options : & [Value] , category : & str) -> Option < ConfigMenu >"
---

Finds the select option with the given category and constructs a menu.

Finds the first `type: "select"` option with the given `category` and constructs a `ConfigMenu`.

# Examples

```rust
use crawrad_agent::chat_config::menu_for;
use serde_json::json;

let opts = vec![json!({
    "type": "select",
    "category": "model",
    "id": "model_opt",
    "name": "Model",
    "currentValue": "m1",
    "options": [{"value": "m1", "name": "Model 1"}]
})];
let menu = menu_for(&opts, "model");
assert!(menu.is_some());
```
