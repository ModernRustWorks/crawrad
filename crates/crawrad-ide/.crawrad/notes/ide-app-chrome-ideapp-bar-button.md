---
id: ide-app-chrome-ideapp-bar-button
kind: doc
target_file: src/ide_app/chrome.rs
line_hint: 198
author: 
created: 
title: "IdeApp::bar_button"
tags: []
signature: "pub (super) fn bar_button (& self , c : toolbar :: Command) -> WidgetRef"
---

The button for a command in the top strip.

The `match` is exhaustive on purpose: a new command in the enum **does not
compile** until it gets its button in the DSL.
