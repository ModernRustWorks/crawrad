---
id: ide-app-status-bar-ideapp-celula-do-zoom
kind: doc
target_file: src/ide_app/status_bar.rs
line_hint: 115
author: 
created: 
title: "IdeApp::celula_do_zoom"
tags: []
signature: "fn celula_do_zoom (& self) -> String"
---

Formats the zoom level status cell showing canvas and interface scales.

**The zoom cell — and there are TWO zooms, which is why it says so.**

The owner looked at the bar after scaling the interface and asked:
*"the zoom is ok, but I don't know if this was supposed to change too when zooming"*,
then *"it's fixed at 100%"*.

He was right to doubt it. The cell showed the CANVAS zoom — the
designer's magnifier, whose `100%` means *the drawing measures what the
application will* — while `Ctrl +` had just moved the INTERFACE. Two
different facts under one label.

# Why the interface only shows up when it is not 100%

Because a cell that reads `100%` on every machine, every session,
teaches nobody anything — and it would be the second number competing
with the one that matters for a form. At the system size it says
nothing; away from it, it says what it is.

The canvas number never leaves: it is the ruler of the designer, and it
is the one the user has to be able to trust at a glance.
