---
id: ide-app-link-ideapp-show-inspector-tab
kind: doc
target_file: src/ide_app/link.rs
line_hint: 1
author:
created:
title: "IdeApp::show_inspector_tab"
tags: []
signature: "pub (super) fn show_inspector_tab (& mut self , cx : & mut Cx , tab : InspectorTab)"
---

Brings a tab of the Inspector to the front, as a click on it would (#851).

The Inspector panel is shown first: a layout that had closed it gets it back,
so the command never lands on a tab nobody can see.

What was being typed is dropped, as when the tab changes by hand.
