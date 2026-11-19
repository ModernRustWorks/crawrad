---
id: ide-app-explorer-actions-ideapp-aplicar-comando-do-inspector
kind: doc
target_file: src/ide_app/explorer_actions.rs
line_hint: 443
author: 
created: 
title: "IdeApp::aplicar_comando_do_inspector"
tags: []
signature: "pub (super) fn aplicar_comando_do_inspector (& mut self , cx : & mut Cx , comando : crawrad_actions :: InspectorCommand ,)"
---

Opens, closes, or folds something inside the Object Inspector.

This is the **only** port: the click on `…`, the click on the chevron,
the `Esc` inside the pad, and the protocol all arrive here, because
they all pass through the same catalog action (`inspector.*`). None of
them touches the field directly.

#### At `self.items = None;`

The Items Editor closes through the SAME action, and that is what makes
its `Esc` work without a second port: `inspector.close_editor` means
*close whichever window editor is open*, not *close the pad*.
