---
id: chat-config-button-label
kind: doc
target_file: src/chat_config.rs
line_hint: 95
author: 
created: 
title: "button_label"
tags: []
signature: "pub fn button_label (menu : & ConfigMenu) -> String"
---

Formats the button label for a configuration menu.

Formats the button label for a menu: the selected item's label (or the first one), followed by ` ▾`.

# Examples

```rust
use crawrad_agent::chat_config::{ConfigMenu, MenuItem, button_label};

let menu = ConfigMenu {
    config_id: "model".to_string(),
    title: "Model".to_string(),
    items: vec![MenuItem {
        value: "fast".to_string(),
        label: "Fast".to_string(),
        selected: true,
    }],
};
assert_eq!(button_label(&menu), "Fast ▾");
```
