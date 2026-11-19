---
id: ide-app-sync-ideapp-follow-save-buttons
kind: doc
target_file: src/ide_app/sync.rs
line_hint: 384
author: 
created: 
title: "IdeApp::follow_save_buttons"
tags: []
signature: "pub (super) fn follow_save_buttons (& mut self , cx : & mut Cx)"
---

Updates toolbar save buttons enabled states based on unsaved changes.

Save and Save All lit only while there is something to save — from the
draw pass, as the tab's dirty dot (`sincronizar` misses keystrokes), and
applied only when the pair changes, so the redraw it asks for ends.
