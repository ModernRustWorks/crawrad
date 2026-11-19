---
id: ide-app-explorer-actions-ideapp-linhas-dos-pacotes
kind: doc
target_file: src/ide_app/explorer_actions.rs
line_hint: 508
author: 
created: 
title: "IdeApp::linhas_dos_pacotes"
tags: []
signature: "pub (super) fn linhas_dos_pacotes (& self) -> Vec < packages :: Row >"
---

The Package Manager rows.

`has_project` is what separates "nothing installed" from "nowhere to
install": without a project, an `Add` button would accept the gesture
and change nothing, which is the dead control that this shell does not
allow to return.
