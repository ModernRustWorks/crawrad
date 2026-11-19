---
id: workspace-update-preferences
kind: doc
target_file: src/workspace.rs
line_hint: 1154
author: 
created: 
title: "update_preferences"
tags: []
signature: "pub fn update_preferences (change : impl FnOnce (& mut Preferences))"
---

Reads, changes and saves the preferences in one step.

Reads, changes and saves the preferences in one step — what every check
box of Options does when it is clicked.

# Examples

```rust,no_run
use crawrad_project::workspace::update_preferences;

// Avoid writing configuration files during doctests.
update_preferences(|p| p.editor_font_size = 12);
```
