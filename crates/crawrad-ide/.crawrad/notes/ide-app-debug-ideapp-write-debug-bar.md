---
id: ide-app-debug-ideapp-write-debug-bar
kind: doc
target_file: src/ide_app/debug.rs
line_hint: 133
author: 
created: 
title: "IdeApp::write_debug_bar"
tags: []
signature: "pub (super) fn write_debug_bar (& mut self , cx : & mut Cx)"
---

Updates control bar button states and glyphs for debugging.

The glyphs of the control bar, and which buttons can act now. A button
that cannot is DIMMED, never hidden (the owner's rule).
