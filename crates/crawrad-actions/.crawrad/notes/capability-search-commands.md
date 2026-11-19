---
id: capability-search-commands
kind: doc
target_file: src/capability.rs
line_hint: 934
author: 
created: 
title: "search_commands"
tags: []
signature: "pub fn search_commands (registry : & CapabilityRegistry , filter : & str) -> Vec < PaletteItem >"
---

The palette's rows for `filter` (a leading `>` is dropped).

# Examples

```rust
use crawrad_actions::capability::{CapabilityRegistry, search_commands};

let reg = CapabilityRegistry::new();
let items = search_commands(&reg, ">save");
assert!(items.is_empty());
```
