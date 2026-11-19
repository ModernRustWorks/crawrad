---
id: ide-app-link-ideapp-event-line-at
kind: doc
target_file: src/ide_app/link.rs
line_hint: 13
author: 
created: 
title: "IdeApp::event_line_at"
tags: []
signature: "pub (super) fn event_line_at (& self , p : DVec2) -> Option < usize >"
---

The Object Inspector lines, from the catalogue and the document.

The registry is the SAME that `designer` loads, for the reason written in
[`IdeApp::linhas_da_paleta`]: a second `Registry::builtin()` would be a
second catalogue to diverge from the first.

The target is that of the runtime drawing the canvas — a property with the prefix of
another target is not from this drawing (ADR 0002).
The line of the `Events` tab under this point, if there is one.

Through the rectangles MEASURED in the last frame, like the rail context menu
hit-test — *screen numbers are asked from the screen*.
