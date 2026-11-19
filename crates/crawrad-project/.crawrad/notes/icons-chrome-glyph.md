---
id: icons-chrome-glyph
kind: doc
target_file: src/icons.rs
line_hint: 264
author: 
created: 
title: "Chrome::glyph"
tags: []
signature: "pub fn glyph (self) -> & 'static str"
---

The Codicons glyph for this role.

Each choice is the one a modern editor makes for the same job, and not a
preference: `files` IS its Explorer, `settings-gear` IS the gear at the
foot of its rail. Two deserve a note, and both notes are the egui
shell's, word for word:

- **Undo** is `discard`, not an `undo`: Codicons has no `undo`, and
  `discard` is the counter-clockwise arrow a modern editor itself uses to undo —
  the exact mirror of `redo`.
- **Inspector** is `symbol-property`, the wrench. It is the icon of the
  Visual Studio Properties window, and the Object Inspector is that
  window.

# Examples

```rust
use crawrad_project::icons::Chrome;

assert!(!Chrome::Save.glyph().is_empty());
```
