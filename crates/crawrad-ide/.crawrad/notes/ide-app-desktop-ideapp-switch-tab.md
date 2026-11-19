---
id: ide-app-desktop-ideapp-switch-tab
kind: doc
target_file: src/ide_app/desktop.rs
line_hint: 14
author: 
created: 
title: "IdeApp::switch_tab"
tags: []
signature: "pub (super) fn switch_tab (& mut self , cx : & mut Cx , to : impl FnOnce (& mut EditorTabs))"
---

Changes the active editor tab while preserving carets.

Changes the tab in front keeping each tab's caret: the one leaving is
remembered, the one arriving lands where it was.
