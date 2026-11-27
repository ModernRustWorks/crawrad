---
id: workspace-preferences-to-text
kind: doc
target_file: src/workspace.rs
line_hint: 1280
author: 
created: 
title: "preferences_to_text"
tags: []
signature: "pub fn preferences_to_text (p : Preferences) -> String"
---

Serializes preferences to text.

# Examples

```rust
use crawrad_project::workspace::{preferences_to_text, Preferences};

let text = preferences_to_text(Preferences::default());
assert!(text.contains("welcome-on-startup=true"));

// A form is written only when there is one, so an empty value is never read back
// as a path and reopened as a missing file.
assert!(!text.contains("last-form="));
```
