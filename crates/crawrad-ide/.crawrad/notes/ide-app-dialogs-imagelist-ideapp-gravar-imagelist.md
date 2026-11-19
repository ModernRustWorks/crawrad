---
id: ide-app-dialogs-imagelist-ideapp-gravar-imagelist
kind: doc
target_file: src/ide_app/dialogs/imagelist.rs
line_hint: 292
author: 
created: 
title: "IdeApp::gravar_imagelist"
tags: []
signature: "pub (in super :: super) fn gravar_imagelist (& mut self , cx : & mut Cx)"
---

Takes the image list to the document — **via the catalog action**.

# Two calls, and why is stated

The egui shell builds ONE `Transaction` with three `SetProp` (the images and the
size pair), and it is the second of the two mutations that the
`crawrad-actions` gate still counts. Not here: `component.set_property` writes
one key at a time, so the size is only sent when it CHANGED — and in normal use
(adding an icon) it is just a single `Ctrl+Z` step, like there.

What is needed to turn into one: an action that accepts MULTIPLE keys in a batch.
It is not improvised here — modifying `component.set_property` means
touching the gate that the entire Object Inspector uses.
