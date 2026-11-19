---
id: ide-app-arrangement-ideapp-empilhar-painel
kind: doc
target_file: src/ide_app/arrangement.rs
line_hint: 254
author: 
created: 
title: "IdeApp::empilhar_painel"
tags: []
signature: "pub (super) fn empilhar_painel (& mut self , cx : & mut Cx , p : Panel , sobre : Panel)"
---

Places the panel in the same tab group as another.

The targeted point is the CENTER of the destination panel's area, which is what
`find_drop_position` reads as `DropPart::Center` — stacking. The same
gesture, and the same code, as dragging a tab to the middle of another.
