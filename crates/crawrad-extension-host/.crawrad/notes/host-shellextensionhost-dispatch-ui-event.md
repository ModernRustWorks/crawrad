---
id: host-shellextensionhost-dispatch-ui-event
kind: doc
target_file: src/host.rs
line_hint: 368
author: 
created: 
title: "ShellExtensionHost::dispatch_ui_event"
tags: []
signature: "pub fn dispatch_ui_event (& mut self , card_id : & str , element_id : & str , event : UiEventKind ,) -> Result < () , ExtensionError >"
---

Dispatch a UI event from the IDE to the subprocess extension owning the card.

# Examples

```rust
use crawrad_extension_api::ui::UiEventKind;
use crawrad_extension_host::ShellExtensionHost;

let mut host = ShellExtensionHost::default();
// No extension owns card `c1`: the event has nowhere to go and is dropped.
assert!(host.dispatch_ui_event("c1", "b1", UiEventKind::Click).is_ok());
```

# Errors

Returns [`ExtensionError`](crawrad_extension_api::error::ExtensionError) if the card's extension cannot be reached.
