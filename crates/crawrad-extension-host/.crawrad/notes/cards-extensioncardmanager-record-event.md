---
id: cards-extensioncardmanager-record-event
kind: doc
target_file: src/cards.rs
line_hint: 123
author: 
created: 
title: "ExtensionCardManager::record_event"
tags: []
signature: "pub fn record_event (& mut self , notif : UiEventNotification)"
---

Record a UI event dispatched from IDE to extension.

# Examples

```rust
use crawrad_extension_api::ui::{UiEventKind, UiEventNotification};
use crawrad_extension_host::ExtensionCardManager;

let mut mgr = ExtensionCardManager::new();
mgr.record_event(UiEventNotification {
    card_id: "c1".into(),
    element_id: "b1".into(),
    event: UiEventKind::Click,
});
assert_eq!(mgr.take_events().len(), 1);
```
