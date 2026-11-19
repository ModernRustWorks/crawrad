---
id: preferred-shortcuts-2
kind: doc
target_file: src/lib.rs
line_hint: 5995
author: 
created: 
title: "preferred_shortcuts"
tags: []
signature: "pub fn preferred_shortcuts (id : & str) -> & 'static [& 'static str]"
---

**All** preferred chords of an action — what the bridge BINDS.

Empty for almost all: only canvas zoom has a pair of keys owned
by another host. See [`PREFERRED_SHORTCUTS`].

# Examples

```rust
use crawrad_actions::preferred_shortcuts;

assert!(!preferred_shortcuts("view.ui_zoom_in").is_empty());
```
