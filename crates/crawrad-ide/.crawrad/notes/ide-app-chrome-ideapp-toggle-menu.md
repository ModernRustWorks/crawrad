---
id: ide-app-chrome-ideapp-toggle-menu
kind: doc
target_file: src/ide_app/chrome.rs
line_hint: 499
author: 
created: 
title: "IdeApp::toggle_menu"
tags: []
signature: "pub (super) fn toggle_menu (& mut self , cx : & mut Cx , menu : Aberto)"
---

Opens `menu`, or closes it when it is the one already open.

Every popup that hangs from a single button (the rail's gear, the
rust-analyzer cell, the Run split button's chevron) answers a second
click on that button the same way: the menu goes away. One function,
so the three buttons cannot drift apart.
