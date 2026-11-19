---
id: chat-config-menuitem
kind: doc
target_file: src/chat_config.rs
line_hint: 16
author: 
created: 
title: "MenuItem"
tags: []
signature: "pub struct MenuItem"
---

A selectable option item in a configuration menu.

# Examples

```rust
use crawrad_agent::chat_config::MenuItem;

let item = MenuItem {
    value: "fast".to_string(),
    label: "Fast".to_string(),
    selected: true,
};
assert!(item.selected);
```
