---
id: editor-chords-navigation-2
kind: doc
target_file: src/editor_chords.rs
line_hint: 116
author: 
created: 
title: "navigation"
tags: []
signature: "pub fn navigation (is : & dyn Fn (& str) -> bool) -> Option < Navigation >"
---

Matches an unhandled navigation key combination.

A `Ctrl` navigation key the framework leaves dead: word left and right,
start and end of the file, `Shift` extending. `is` says whether a chord
name is the key pressed.

`Ctrl` is the primary modifier, `Cmd` too: every arm of the framework's own
editor treats `control` and `logo` as one (`code_editor.rs:687`), and a
different rule here would make `Cmd` work for font size and not for the
cursor, on the same keyboard.

# Examples

```rust
use crawrad_host::editor_chords::{navigation, Motion};

let nav = navigation(&|chord| chord == "Ctrl+Left");
assert_eq!(nav.map(|n| n.motion), Some(Motion::WordLeft));
```
