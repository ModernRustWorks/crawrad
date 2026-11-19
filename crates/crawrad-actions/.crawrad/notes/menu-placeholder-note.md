---
id: menu-placeholder-note
kind: doc
target_file: src/menu.rs
line_hint: 463
author: 
created: 
title: "placeholder_note"
tags: []
signature: "pub fn placeholder_note (label : & str) -> Option < & 'static str >"
---

One row of the panel menu, with the catalogue answering for label and state.

`label` overrides `Action::label` only where the catalogue cannot know the
wording: `layout.move_panel` is one action for three destinations, and
*"Move panel to side"* is not a row anyone can click on purpose.
What a row of the gear's or the tabs' `...` menu with no wire yet says when
it runs — the sentence is all it does until a catalogue action takes it.

# Examples

```rust
use crawrad_actions::menu::placeholder_note;

assert!(placeholder_note("Profiles").is_some());
assert!(placeholder_note("Nonexistent").is_none());
```
