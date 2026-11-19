---
id: chat-config-configmenu
kind: doc
target_file: src/chat_config.rs
line_hint: 8
author: 
created: 
title: "ConfigMenu"
tags: []
signature: "pub struct ConfigMenu"
---

A structured dropdown menu derived from an ACP agent's `configOptions` entry.

# Examples

```rust
use crawrad_agent::chat_config::{ConfigMenu, MenuItem};

let menu = ConfigMenu {
    config_id: "model".to_string(),
    title: "Model".to_string(),
    items: vec![MenuItem {
        value: "gpt-4".to_string(),
        label: "GPT-4".to_string(),
        selected: true,
    }],
};
assert_eq!(menu.items.len(), 1);
```
