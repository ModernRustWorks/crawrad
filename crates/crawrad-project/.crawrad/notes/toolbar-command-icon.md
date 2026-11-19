---
id: toolbar-command-icon
kind: doc
target_file: src/toolbar.rs
line_hint: 51
author: 
created: 
title: "Command::icon"
tags: []
signature: "pub fn icon (self) -> icon :: Chrome"
---

The icon role this button draws.

It is a `Chrome` and not a glyph: the set is swapped in one place, and
the drawing is the same one the rail and the egui shell use for the
same job.

# Examples

```rust
use crawrad_project::icons::Chrome;
use crawrad_project::toolbar::Command;

assert_eq!(Command::Run.icon(), Chrome::Start);
```
