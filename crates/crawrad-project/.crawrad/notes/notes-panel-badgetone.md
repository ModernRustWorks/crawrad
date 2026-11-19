---
id: notes-panel-badgetone
kind: doc
target_file: src/notes_panel.rs
line_hint: 14
author: 
created: 
title: "BadgeTone"
tags: []
signature: "pub enum BadgeTone"
---

Why a row asks for attention, or a neutral mark.

# Examples

```rust
use crawrad_project::notes_panel::BadgeTone;

assert_ne!(BadgeTone::Warn, BadgeTone::Done);
```
