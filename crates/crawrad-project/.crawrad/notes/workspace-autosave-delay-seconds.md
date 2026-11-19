---
id: workspace-autosave-delay-seconds
kind: doc
target_file: src/workspace.rs
line_hint: 487
author: 
created: 
title: "AutoSave::DELAY_SECONDS"
tags: []
signature: "pub const DELAY_SECONDS : f64 = 1.0"
---

The pause after the last keystroke before [`AutoSave::AfterDelay`] writes.

# Examples

```rust
use crawrad_project::workspace::AutoSave;

assert_eq!(AutoSave::DELAY_SECONDS, 1.0);
```
