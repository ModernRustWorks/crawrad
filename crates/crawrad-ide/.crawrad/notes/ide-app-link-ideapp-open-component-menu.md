---
id: ide-app-link-ideapp-open-component-menu
kind: doc
target_file: src/ide_app/link.rs
line_hint: 1
author:
created:
title: "IdeApp::open_component_menu"
tags: []
signature: "pub (super) fn open_component_menu (& mut self , cx : & mut Cx , at : DVec2) -> bool"
---

Right-click on a component of the canvas: select it and open its menu (#851).

The form itself, the code view and the empty canvas answer `false`, and the
panel's own menu opens instead.
