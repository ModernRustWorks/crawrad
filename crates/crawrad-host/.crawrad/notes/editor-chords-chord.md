---
id: editor-chords-chord
kind: doc
target_file: src/editor_chords.rs
line_hint: 70
author: 
created: 
title: "Chord"
tags: []
signature: "type Chord = (Option < bool > , & 'static str , Command)"
---

Chord definition with keymap scheme, shortcut string, and command.

Which scheme a chord belongs to (`None`: both, `Some(true)`: Classic), its
name, and the command. The modifiers in the name must match exactly.
