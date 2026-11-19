---
id: note-dialog-labelpaint
kind: doc
target_file: src/note_dialog.rs
line_hint: 253
author: 
created: 
title: "LabelPaint"
tags: []
signature: "pub type LabelPaint < 'a > = (& 'static str , & 'a str , bool , Option < Tone >)"
---

Label rendering information for the note dialog shell.

One label of the dialog as the shell paints it: control name, text,
whether it shows, and its tone (`None` keeps the theme's colour).

# Examples

```rust
use crawrad_project::note_dialog::LabelPaint;

let l: LabelPaint = ("control", "text", true, None);
assert!(l.2);
```
