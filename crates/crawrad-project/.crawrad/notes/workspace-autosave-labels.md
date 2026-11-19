---
id: workspace-autosave-labels
kind: doc
target_file: src/workspace.rs
line_hint: 500
author: 
created: 
title: "AutoSave::labels"
tags: []
signature: "pub fn labels () -> Vec < String >"
---

Every label, in [`AutoSave::ALL`] order.

# Examples

```rust
use crawrad_project::workspace::AutoSave;

assert_eq!(AutoSave::labels().len(), 4);
```
