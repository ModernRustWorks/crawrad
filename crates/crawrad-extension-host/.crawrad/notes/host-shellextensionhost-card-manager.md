---
id: host-shellextensionhost-card-manager
kind: doc
target_file: src/host.rs
line_hint: 363
author: 
created: 
title: "ShellExtensionHost::card_manager"
tags: []
signature: "pub fn card_manager (& self) -> Arc < Mutex < ExtensionCardManager > >"
---

Access the card manager handling interactive cards and dynamic UI panels.

# Examples

```rust
use crawrad_extension_host::ShellExtensionHost;

let host = ShellExtensionHost::default();
let cm = host.card_manager();
assert!(cm.lock().unwrap().list_cards().is_empty());
```
