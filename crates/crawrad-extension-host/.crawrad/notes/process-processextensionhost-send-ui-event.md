---
id: process-processextensionhost-send-ui-event
kind: doc
target_file: src/process.rs
line_hint: 702
author: 
created: 
title: "ProcessExtensionHost::send_ui_event"
tags: []
signature: "pub fn send_ui_event (& mut self , card_id : & str , element_id : & str , event : UiEventKind ,) -> Result < () , ExtensionError >"
---

Send an interactive UI event to the extension subprocess owning the card.

# Examples

```rust
use crawrad_extension_api::ui::UiEventKind;
use crawrad_extension_host::{ExtensionCardManager, ProcessExtensionHost};
use std::sync::{Arc, Mutex};

let cm = Arc::new(Mutex::new(ExtensionCardManager::new()));
let mut host = ProcessExtensionHost::new(None, cm);
assert!(host.send_ui_event("c1", "e1", UiEventKind::Click).is_ok());
```

# Errors

Returns [`ExtensionError`](crawrad_extension_api::error::ExtensionError) if forwarding the event to the extension fails.

# Panics

Panics if the internal card manager lock is poisoned.
