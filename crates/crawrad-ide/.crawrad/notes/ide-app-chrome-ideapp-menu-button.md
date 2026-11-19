---
id: ide-app-chrome-ideapp-menu-button
kind: doc
target_file: src/ide_app/chrome.rs
line_hint: 210
author: 
created: 
title: "IdeApp::menu_button"
tags: []
signature: "pub (super) fn menu_button (& self , m : menu :: Menu) -> WidgetRef"
---

The button for a menu in the bar.

The `match` is exhaustive on purpose: a new menu in the enum **does not compile**
until it gets its button in the DSL. It is the difference between this and a
hand-written list of widget paths, which would be born outdated.
