---
id: ide-app-chrome-ideapp-point-in-window-drag
kind: doc
target_file: src/ide_app/chrome.rs
line_hint: 519
author: 
created: 
title: "IdeApp::point_in_window_drag"
tags: []
signature: "pub (super) fn point_in_window_drag (& self , cx : & Cx , pos : DVec2) -> bool"
---

Reports whether the point `pos` is over a draggable area of the title bar.

# Why the shell needs to respond

Makepad by default only responds to `WindowDragQuery` in its internal `caption_bar`
(`makepad-widgets-1.0.0/src/window.rs:377-396`). Since the IDE
draws its own bar (`top_titlebar`) and hides the framework's
(`caption_bar.set_visible(false)`), Makepad's visibility test
always fails and the window was left without `HTCAPTION` on Windows — the user
could not drag the window by the title bar.

# Excluded areas

Any button or interactive control in the bar must NOT trigger a drag,
so that normal clicks continue reaching them:
- The 8 menu buttons (`File`, `Edit`, `View`, `Project`, `Run`, `Component`, `Tools`, `Help`)
- The action buttons (`Undo`, `Redo`, `Save`, `Play`, `Preview`, `+ AI`)
- The window control buttons (`Min`, `Max`, `Restore`, `Close`)

All the rest of `top_titlebar` (empty spaces and the central project title)
responds affirmatively, allowing dragging, maximizing by double-click,
and opening the system menu via right-click.
