---
id: ide-app-desktop-ideapp-place-startup-window
kind: doc
target_file: src/ide_app/desktop.rs
line_hint: 149
author: 
created: 
title: "IdeApp::place_startup_window"
tags: []
signature: "pub (super) fn place_startup_window (& mut self) -> bool"
---

Restores saved window bounds at startup if on-screen.

The window the desktop kept, placed at startup when it was not
maximized and still fits a screen; `false` leaves the maximize to run.
