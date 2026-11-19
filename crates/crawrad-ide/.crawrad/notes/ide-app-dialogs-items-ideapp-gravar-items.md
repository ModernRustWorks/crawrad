---
id: ide-app-dialogs-items-ideapp-gravar-items
kind: doc
target_file: src/ide_app/dialogs/items.rs
line_hint: 169
author: 
created: 
title: "IdeApp::gravar_items"
tags: []
signature: "pub (in super :: super) fn gravar_items (& mut self , cx : & mut Cx)"
---

Takes the edited collection to the document — **via the catalog action**.

No `Transaction` is born here: the value goes out through
`component.set_property`, the same gate as the keyboard, the menu, and an
agent. It is the difference between `Apply` and `OK`: the former saves and
leaves the window open, the latter saves and closes.
