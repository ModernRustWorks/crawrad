---
id: workspace-save-preferences
kind: doc
target_file: src/workspace.rs
line_hint: 1146
author: 
created: 
title: "save_preferences"
tags: []
signature: "pub fn save_preferences (p : Preferences)"
---

Saves preferences to disk.

Saves. Fails silently, like recents: not being able to remember a checked box
is an annoyance, and interrupting the user to announce it would be worse
than the problem.

# Examples

```rust,no_run
use crawrad_project::workspace::{save_preferences, Preferences};

// Avoid writing configuration files during doctests.
save_preferences(Preferences::default());
```
