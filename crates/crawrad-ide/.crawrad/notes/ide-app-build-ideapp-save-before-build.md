---
id: ide-app-build-ideapp-save-before-build
kind: doc
target_file: src/ide_app/build.rs
line_hint: 329
author: 
created: 
title: "IdeApp::save_before_build"
tags: []
signature: "pub (super) fn save_before_build (& mut self , cx : & mut Cx) -> bool"
---

Saves what is typed before any build; `false` means the build must stop.

**WHAT IS TYPED GOES TO DISK FIRST**, and that is correctness, not courtesy.

The generation pre-pass READS the unit from disk to know which handlers exist, and the generator
rewrites on top of it. With a typed and unsaved line, it would evaluate a file that is not
what is on screen and would return the old version plus stubs — the user's work
erased by a save they did not ask for. That is what every IDE does before compiling, and it is
what `Run` in a classic RAD IDE does. See [`editor_buffer`].

AND IF SAVING FAILS, IT DOES NOT GENERATE. The tab mark remains there (`save_editor_file` does not
remove it when the disk refuses), and that mark is what answers here — generating on top would be the same
wipeout, only with the IDE claiming everything succeeded.
