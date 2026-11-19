---
id: host-shellextensionhost-contributed-cards
kind: doc
target_file: src/host.rs
line_hint: 339
author: 
created: 
title: "ShellExtensionHost::contributed_cards"
tags: []
signature: "pub fn contributed_cards (& self) -> Vec < & CardContribution >"
---

List all cards contributed across registered extension manifests.

# Examples

```rust
use crawrad_extension_host::ShellExtensionHost;

let host = ShellExtensionHost::default();
assert!(host.contributed_cards().is_empty());
```
