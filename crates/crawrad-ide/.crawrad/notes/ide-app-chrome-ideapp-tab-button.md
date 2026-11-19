---
id: ide-app-chrome-ideapp-tab-button
kind: doc
target_file: src/ide_app/chrome.rs
line_hint: 594
author: 
created: 
title: "IdeApp::tab_button"
tags: []
signature: "pub (super) fn tab_button (& self , aba : InspectorTab) -> WidgetRef"
---

The button for an Inspector tab.

The `match` is exhaustive on purpose: a new tab in the enum **does not compile**
until it gets its button. It is the difference between this and a hand-written
list of widget paths, which would be born outdated.
