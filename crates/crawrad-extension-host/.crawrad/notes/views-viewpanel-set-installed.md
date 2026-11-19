---
id: views-viewpanel-set-installed
kind: doc
target_file: src/views.rs
line_hint: 206
author: 
created: 
title: "ViewPanel::set_installed"
tags: []
signature: "pub fn set_installed (& mut self , now : Vec < Installed >) -> bool"
---

The installed extensions now; `true` when they changed.

# Examples

```rust
use crawrad_extension_host::views::ViewPanel;

let mut panel = ViewPanel::default();
assert!(!panel.set_installed(Vec::new()));
```
