---
id: ide-app-link-ideapp-bind-event
kind: doc
target_file: src/ide_app/link.rs
line_hint: 52
author: 
created: 
title: "IdeApp::bind_event"
tags: []
signature: "pub (super) fn bind_event (& mut self , cx : & mut Cx , no : NodeId , evento : & str , handler : & str)"
---

Binds the handler to the event — and makes the panel line show it.

**No `Transaction` is born here**: what binds is the catalogue action
`event.bind`, the same door that the keyboard, the menu, and an agent use
(ADR 0029). It, and not this shell, decides that empty text unbinds
and that an unknown event is rejected with its phrase.

THE LIST REDRAW is what separates naming from the blank on screen:
`sincronizar` rewrites the header and the bar, and its `self.ui.redraw`
does not reach a `PortalList` inside a dock item. Without this
line, the handler went to the document and the `Events` tab line remained
blank — measured on screen on 09/16/2026.
