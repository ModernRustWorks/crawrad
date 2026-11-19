---
id: workspace-preferences-from-text
kind: doc
target_file: src/workspace.rs
line_hint: 1161
author: 
created: 
title: "preferences_from_text"
tags: []
signature: "pub fn preferences_from_text (text : & str) -> Preferences"
---

Parses preferences from configuration text.

The file text becomes preferences. Unknown keys are ignored — this
file is editable by hand, and a key from a future version cannot break
reading the others.

# Examples

```rust
use crawrad_project::workspace::preferences_from_text;

let p = preferences_from_text("welcome-on-startup=true\n");
assert!(p.welcome_on_startup);
```
