---
id: ide-app-ui-zoom-ideapp-set-ide-zoom
kind: doc
target_file: src/ide_app/ui_zoom.rs
line_hint: 58
author: 
created: 
title: "IdeApp::set_ide_zoom"
tags: []
signature: "pub (super) fn set_ide_zoom (& mut self , cx : & mut Cx , fator : f64)"
---

The interface zoom set to `fator` (1.0 is 100%).

## Inside `set_ide_zoom`: `crawrad_os::nudge_resize();`

WRITTEN AND NOT HAPPENED: the framework only applies `dpi_override`
when a geometry change ARRIVES. Without the nudge, the field changes
and the screen does not — see the header of this module.
