---
id: ide-app-arrangement-ideapp-load-arrangement
kind: doc
target_file: src/ide_app/arrangement.rs
line_hint: 477
author: 
created: 
title: "IdeApp::load_arrangement"
tags: []
signature: "pub (super) fn load_arrangement (& mut self , cx : & mut Cx)"
---

Restores the saved layout, whenever there is one that can be trusted.

Whatever can**not** be trusted is completely discarded and the status bar says
so — see [`layout::decode`]. Applying half a layout would leave the window
without a panel and without an explanation.

## Inside `load_arrangement`: `if tinha_pacotes {`

THE RESTORED PACKAGE MANAGER also reads the manifest.

Defect measured on screen on 16/09/2026: whoever closed the IDE with
the panel open found it upon return saying
`Installed (0)` for a project with dependencies. The
`abrir` lives in `show_panel`, and the saved layout
returns the tab without going through there — the only path where the
panel is born visible without anyone having requested it in that
session.
