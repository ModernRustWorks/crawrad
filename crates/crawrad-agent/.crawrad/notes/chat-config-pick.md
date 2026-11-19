---
id: chat-config-pick
kind: doc
target_file: src/chat_config.rs
line_hint: 272
author: 
created: 
title: "pick"
tags: []
signature: "pub fn pick (live : & mut crate :: acp_live :: LiveAgent , config : & mut crate :: acp :: AcpConfig , menu : & ConfigMenu , index : usize ,) -> Result < Value , String >"
---

Applies a selected menu item choice.

Applies item `index` of `menu`, the one path for a click and for MCP: an agent
option goes through [`apply_choice`]; a catalogue model is saved as the
provider's model, as before #402.

# Errors

Returns an error if the item index is invalid or applying the choice fails.

# Examples

```rust
use crawrad_agent::chat_config::{ConfigMenu, pick};
use crawrad_agent::{AcpConfig, LiveAgent};

let mut live = LiveAgent::new();
let mut cfg = AcpConfig::default();
let menu = ConfigMenu {
    config_id: "unknown".to_string(),
    title: "Test".to_string(),
    items: vec![],
};
assert!(pick(&mut live, &mut cfg, &menu, 0).is_err());
```
