---
id: chat-config-button-labels
kind: doc
target_file: src/chat_config.rs
line_hint: 292
author: 
created: 
title: "button_labels"
tags: []
signature: "pub fn button_labels (live : & crate :: acp_live :: LiveAgent , config : & crate :: acp :: AcpConfig ,) -> (String , Option < String >)"
---

Returns the button labels for model and effort selection.

The text of the model and effort buttons; the effort one is `None` (hidden)
when the agent offers no `thought_level`.

# Examples

```rust
use crawrad_agent::chat_config::button_labels;
use crawrad_agent::{AcpConfig, LiveAgent};

let live = LiveAgent::new();
let cfg = AcpConfig::default();
let (model, effort) = button_labels(&live, &cfg);
assert!(!model.is_empty());
```
