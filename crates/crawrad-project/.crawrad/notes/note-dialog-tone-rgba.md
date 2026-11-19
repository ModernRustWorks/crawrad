---
id: note-dialog-tone-rgba
kind: doc
target_file: src/note_dialog.rs
line_hint: 46
author: 
created: 
title: "Tone::rgba"
tags: []
signature: "pub fn rgba (self) -> u32"
---

Returns the RGBA colour token for this tone.

RGBA, the theme's text primary, text tertiary, warning, ACP chip and
error tokens.

# Examples

```rust
use crawrad_project::note_dialog::Tone;

assert_ne!(Tone::Plain.rgba(), 0);
```
