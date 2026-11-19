---
id: ide-app-slot-path
kind: doc
target_file: src/ide_app.rs
line_hint: 1100
author: 
created: 
title: "slot_path"
tags: []
signature: "pub (crate) fn slot_path (slot : inspector :: Slot) -> & 'static [LiveId]"
---

The path, in the DSL, of the widget for each [`inspector::Slot`].

**Exhaustive on purpose**: a new slot in the enum will not compile until it has a path
here, and this is what makes it impossible for an editor to name a place that the DSL does
not have. A `LiveId::from_str` on an arbitrary name would silently return `WidgetRef::empty()`,
which is the widget that neither draws nor complains — the family of
defects that the DSL gate exists to close.

It is a free-standing function and not a method for the same reason as [`IdeApp::escrever_linha`]:
it does not touch `self`.
