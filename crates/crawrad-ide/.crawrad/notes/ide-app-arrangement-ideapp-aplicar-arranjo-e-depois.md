---
id: ide-app-arrangement-ideapp-aplicar-arranjo-e-depois
kind: doc
target_file: src/ide_app/arrangement.rs
line_hint: 1
author: 
created: 
title: "IdeApp::aplicar_arranjo_e_depois"
tags: []
signature: "pub (super) fn aplicar_arranjo_e_depois (& mut self , cx : & mut Cx , comando : crawrad_actions :: LayoutCommand)"
---

Applies one layout command to the dock, without rewriting the panel chrome.

## Inside `aplicar_arranjo_e_depois`: `self.show_center(cx, self.host.center());`

`load_state` RECREATES panel widgets (`dock.rs:983`),
and the center panel returns with the DSL's `visible` — the
design. Without this line, a `Reset Layout` triggered from inside
`Design Code` would leave the screen showing the design and the
`centro` field saying `FormSource`: the subsequent `F12` would compute
from a state that the screen does not have.

## Inside `aplicar_arranjo_e_depois`: `self.status = Some(format!(`

`DockItem` DOES NOT have a `Float` variant (`dock.rs:315-334`:
`Splitter`, `Tabs`, `Tab`), and floating inside the window is
OURS (ADR 0014, Amendment 8) — an overlay layer, just like this same
shell's menu already does. It has not been written yet, and what lacks
wiring SAYS that it lacks it.
