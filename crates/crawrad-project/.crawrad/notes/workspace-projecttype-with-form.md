---
id: workspace-projecttype-with-form
kind: doc
target_file: src/workspace.rs
line_hint: 1402
author: 
created: 
title: "ProjectType::with_form"
tags: []
signature: "pub fn with_form (self) -> bool"
---

Whether this project type starts with a form.

Born with a form?

Only GUI. The other two land in the *no document open* state right after
creation — the **same** state as the first run and the folder without a form,
not a third similar state.

# Examples

```rust
use crawrad_project::workspace::ProjectType;

assert!(ProjectType::Gui.with_form());
assert!(!ProjectType::Console.with_form());
```
