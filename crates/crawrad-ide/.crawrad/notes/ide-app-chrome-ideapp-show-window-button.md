---
id: ide-app-chrome-ideapp-show-window-button
kind: doc
target_file: src/ide_app/chrome.rs
line_hint: 23
author: 
created: 
title: "IdeApp::show_window_button"
tags: []
signature: "pub (super) fn show_window_button (& mut self , cx : & mut Cx)"
---

Updates the window maximize or restore button.

Writes what the DSL intentionally left blank: the label for the Inspector
tabs and the gear icon.

**Why blank in the DSL:** both texts have an owner in Rust — the pair
of tabs is [`InspectorTab::ALL`] and the icons are [`icon::Chrome`]. Written
in the DSL, they would be the same information in two places, and the DSL is
precisely the place where divergence does not break the build (see the module
note on the central trap of Makepad).
Which of the two window buttons is in view — maximize or restore.

They occupy the same place in the bar and never appear together: what is seen
is always what the next click DOES.

**The declared limit:** maximizing from the outside — double clicking the
system bar, `Win+Arrow` — does not pass through here, and the icon keeps saying
the opposite until the next click. The framework emits nothing that distinguishes
maximized from restored (see the [`IdeApp::maximizada`] field).
