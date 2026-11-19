---
id: workspace-projecttype-all
kind: doc
target_file: src/workspace.rs
line_hint: 1359
author: 
created: 
title: "ProjectType::ALL"
tags: []
signature: "pub const ALL : & 'static [Self] = & [Self :: Gui , Self :: Console , Self :: Library]"
---

All three, in the order they appear in the dialog.

# Examples

```rust
use crawrad_project::workspace::ProjectType;

assert_eq!(ProjectType::ALL.len(), 3);
```
