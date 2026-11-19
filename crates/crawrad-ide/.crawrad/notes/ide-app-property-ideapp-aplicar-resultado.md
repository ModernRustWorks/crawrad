---
id: ide-app-property-ideapp-aplicar-resultado
kind: doc
target_file: src/ide_app/property.rs
line_hint: 83
author: 
created: 
title: "IdeApp::aplicar_resultado"
tags: []
signature: "pub (super) fn aplicar_resultado (& mut self , cx : & mut Cx , resultado : inspector :: Outcome , no : Option < & NodeId > , key : & str ,)"
---

What the shell does with what an editor returned.

The four outputs of [`inspector::Outcome`] in one place, because the
four arrive from three different gestures — typing, checking, choosing — and
three copies would be three chances for one of them to swallow the rejection.

**No `Transaction` is born here**: the value goes through
`component.set_property` and the rest goes through the action id that the editor
itself named — the same door that the menu, the keyboard, and the protocol use.
