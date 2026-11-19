---
id: ide-app-desktop-ideapp-restore-desktop-tabs
kind: doc
target_file: src/ide_app/desktop.rs
line_hint: 124
author: 
created: 
title: "IdeApp::restore_desktop_tabs"
tags: []
signature: "pub (super) fn restore_desktop_tabs (& mut self , cx : & mut Cx)"
---

Opens kept desktop tabs once the dock centre is drawn.

The kept tabs, opened once the dock has drawn the centre: before that
the editor found is the one the dock is about to replace, and the new
one's empty text would become each file's unsaved buffer.

## Inside `restore_desktop_tabs`: `self.abas.show_design();`

The canvas in front first, so no kept caret is taken for the
caret of the tab the loop left in front.
