---
id: workspace-read-preferences
kind: doc
target_file: src/workspace.rs
line_hint: 1135
author: 
created: 
title: "read_preferences"
tags: []
signature: "pub fn read_preferences () -> Preferences"
---

Reads preferences from disk.

Preferences from disk. Missing file is the NORMAL case — first
run —, and it returns the default instead of an error.

# Examples

```rust
use crawrad_project::workspace::read_preferences;

let p = read_preferences();
assert!(p.editor_font_size >= 8);
```
