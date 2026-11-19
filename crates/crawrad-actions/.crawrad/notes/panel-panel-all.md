---
id: panel-panel-all
kind: doc
target_file: src/panel.rs
line_hint: 67
author: 
created: 
title: "Panel::ALL"
tags: []
signature: "pub const ALL : & 'static [Self] = & [Self :: Explorer , Self :: Components , Self :: Inspector , Self :: Packages , Self :: Design , Self :: Chat , Self :: Terminal , Self :: Debug , Self :: References , Self :: Problems , Self :: Extensions , Self :: Notes , Self :: Tests , Self :: TestOutput , Self :: FormCheck ,]"
---

Every panel, in the order a fresh layout lays them out left to right.

# Examples

```rust
use crawrad_actions::Panel;

assert_eq!(Panel::ALL.len(), 15);
```
