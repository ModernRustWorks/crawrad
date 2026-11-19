---
id: ide-app-explorer-actions-ideapp-follow-unsaved
kind: doc
target_file: src/ide_app/explorer_actions.rs
line_hint: 84
author: 
created: 
title: "IdeApp::follow_unsaved"
tags: []
signature: "pub (super) fn follow_unsaved (& mut self , cx : & mut Cx)"
---

Redraws the Explorer when unsaved tab states change.

After every event (#417): a tab that turned unsaved or saved redraws the
Explorer — asked from the draw itself, the redraw was lost in the frame.
