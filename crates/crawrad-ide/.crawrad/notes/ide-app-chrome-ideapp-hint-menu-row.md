---
id: ide-app-chrome-ideapp-hint-menu-row
kind: doc
target_file: src/ide_app/chrome.rs
line_hint: 1
author:
created:
title: "IdeApp::hint_menu_row"
tags: []
signature: "pub (super) fn hint_menu_row (& mut self , cx : & mut Cx , at : DVec2)"
---

The tooltip of a greyed menu row: why it cannot run now.

"<label> needs <reason>" (the sentence the MCP refusal gives) while the
pointer rests on a disabled row of the open menu or submenu; gone when it
leaves the row and when the menu closes. Enabled rows get no tooltip.
