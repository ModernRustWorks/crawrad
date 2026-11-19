---
id: ide-app-view-state-ideapp-abrir-no-editor
kind: doc
target_file: src/ide_app/view_state.rs
line_hint: 235
author: 
created: 
title: "IdeApp::abrir_no_editor"
tags: []
signature: "pub (crate) fn abrir_no_editor (& mut self , cx : & mut Cx , arquivo : std :: path :: PathBuf)"
---

**Opens a file in the editor**, or brings the open one forward.

It is the single entryway: `F12`, the double-click leading to the handler (ADR 0015),
and the click in the Explorer all arrive here. Two entryways would be two
chances to open the same unit in two tabs.
