---
id: ide-app-chrome-ideapp-point-in-menu
kind: doc
target_file: src/ide_app/chrome.rs
line_hint: 498
author: 
created: 
title: "IdeApp::point_in_menu"
tags: []
signature: "pub (super) fn point_in_menu (& self , cx : & Cx , p : DVec2) -> bool"
---

Checks whether the point is over the menu or its bar.

Is the point over the menu bar or over the open panel?

It is what separates "clicked outside, close" from "clicked inside, let the widget
respond". Without the bar taken into account, clicking on `Edit` with `File` open
would close and reopen in the same gesture — and the menu would flicker without switching.
