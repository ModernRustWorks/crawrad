---
id: cards-extensioncardmanager-take-events
kind: doc
target_file: src/cards.rs
line_hint: 128
author: 
created: 
title: "ExtensionCardManager::take_events"
tags: []
signature: "pub fn take_events (& mut self) -> Vec < UiEventNotification >"
---

Drain all recorded events.

# Examples

```rust
use crawrad_extension_host::ExtensionCardManager;

let mut mgr = ExtensionCardManager::new();
assert!(mgr.take_events().is_empty());
```
