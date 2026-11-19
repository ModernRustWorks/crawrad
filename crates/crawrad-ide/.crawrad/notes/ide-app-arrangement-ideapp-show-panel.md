---
id: ide-app-arrangement-ideapp-show-panel
kind: doc
target_file: src/ide_app/arrangement.rs
line_hint: 116
author: 
created: 
title: "IdeApp::show_panel"
tags: []
signature: "pub (super) fn show_panel (& mut self , cx : & mut Cx , p : Panel)"
---

Opens the panel's tab, creating it if it had been closed, and focuses it.

The group in which it is born is that of the DEFAULT arrangement — this is what the user
expects from a panel that they closed themselves. If they had moved it before
closing, the old location is lost, and that is a declared limitation: the `Dock`
forgets the group of a closed tab (`dock.rs:684-700`).

## Inside `show_panel`: `if p == Panel::Packages {`

THE PACKAGE MANAGER READS THE MANIFEST when entering the scene, and not before.

`abrir` runs `cargo metadata` and WAITS (`launcher::wait`), which
costs a good hundred milliseconds of child process. Doing this at
startup would charge that time to anyone who will never open the panel; done
here, it is paid by whoever requested it — and it is the same moment when the egui
shell pays it, when its window opens.

## Inside `show_panel`: `if p == Panel::Design {`

The tab was born JUST NOW, and with it a new widget. If it is the
design tab, the document must return inside it — see
[`IdeApp::repovoar_paineis`].
