---
id: ide-app-class-icon-strip-ideapp-class-icon-texture
kind: doc
target_file: src/ide_app/class_icon_strip.rs
line_hint: 52
author: 
created: 
title: "IdeApp::class_icon_texture"
tags: []
signature: "pub (super) fn class_icon_texture (& mut self , cx : & mut Cx) -> Texture"
---

Returns the icon strip texture at current window DPI.

The icon strip at the DPI the window is drawn in — zoom included, since
`window_geom.dpi_factor` is where the zoom override lands.
