---
id: workspace-autosave-from-key
kind: doc
target_file: src/workspace.rs
line_hint: 520
author: 
created: 
title: "AutoSave::from_key"
tags: []
signature: "pub fn from_key (s : & str) -> Self"
---

Back from `preferences.txt`; anything unknown is [`AutoSave::Off`].

# Examples

```rust
use crawrad_project::workspace::AutoSave;

assert_eq!(AutoSave::from_key("off"), AutoSave::Off);
assert_eq!(AutoSave::from_key("unknown"), AutoSave::Off);
```
