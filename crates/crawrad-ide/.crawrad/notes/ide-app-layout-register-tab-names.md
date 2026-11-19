---
id: ide-app-layout-register-tab-names
kind: doc
target_file: src/ide_app/layout.rs
line_hint: 63
author: 
created: 
title: "register_tab_names"
tags: []
signature: "pub fn register_tab_names ()"
---

Registers human-readable names for all panel tabs.

Teaches the id table the name of every tab, so the saved arrangement is
readable text instead of a wall of numbers.

Called once from `LiveRegister::live_register`, which is the earliest hook
this shell has and runs before anything is serialised.

# Examples

```rust,ignore
// binary crate: doctests cannot reach main.rs items
register_tab_names();
```
