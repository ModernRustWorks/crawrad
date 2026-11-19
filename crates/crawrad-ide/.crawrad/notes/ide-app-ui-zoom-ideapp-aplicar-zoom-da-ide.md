---
id: ide-app-ui-zoom-ideapp-aplicar-zoom-da-ide
kind: doc
target_file: src/ide_app/ui_zoom.rs
line_hint: 48
author: 
created: 
title: "IdeApp::aplicar_zoom_da_ide"
tags: []
signature: "pub (super) fn aplicar_zoom_da_ide (& mut self , cx : & mut Cx , pedido : crawrad_actions :: Zoom)"
---

Adjusts the interface zoom by one discrete zoom step.

**Applies one step of interface zoom.**

The factor multiplies the desktop's own DPI, and that is what keeps the
IDE honest on a scaled screen: at 150% Windows scaling and 110% ours,
the window asks for 1.65 — the desktop's preference is not thrown away.
