---
id: chat-config-menu-or-catalogue
kind: doc
target_file: src/chat_config.rs
line_hint: 241
author: 
created: 
title: "menu_or_catalogue"
tags: []
signature: "pub fn menu_or_catalogue (live : & crate :: acp_live :: LiveAgent , config : & crate :: acp :: AcpConfig , category : & str ,) -> Option < ConfigMenu >"
---

Returns the menu for the specified category or the fallback catalogue.

The menu the chat shows for `category`: the agent's own; for `model`, failing
that, the IDE's catalogue for the active provider.

# Examples

```rust
use crawrad_agent::chat_config::menu_or_catalogue;
use crawrad_agent::{AcpConfig, LiveAgent};

let live = LiveAgent::new();
let cfg = AcpConfig::default();
let menu = menu_or_catalogue(&live, &cfg, "model");
assert!(menu.is_some());
```
