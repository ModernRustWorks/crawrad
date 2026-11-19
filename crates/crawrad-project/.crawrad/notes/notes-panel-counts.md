---
id: notes-panel-counts
kind: doc
target_file: src/notes_panel.rs
line_hint: 56
author: 
created: 
title: "Counts"
tags: []
signature: "pub struct Counts"
---

Counts for the chips and the alert toggle.

Counts for the chips and the "In alert" toggle (over ALL notes, independent of filters).

# Examples

```rust
use crawrad_project::notes_panel::Counts;

let counts = Counts::default();
assert_eq!(counts.all, 0);
```
