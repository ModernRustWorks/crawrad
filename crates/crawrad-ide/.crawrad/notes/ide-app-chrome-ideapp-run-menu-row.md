---
id: ide-app-chrome-ideapp-run-menu-row
kind: doc
target_file: src/ide_app/chrome.rs
line_hint: 413
author: 
created: 
title: "IdeApp::run_menu_row"
tags: []
signature: "pub (super) fn run_menu_row (& mut self , cx : & mut Cx , linha : & menu :: Row)"
---

Executes the action of an active menu row.

Fecha o menu aberto, se houver, e diz se fechou alguma coisa.
A row of the open menu runs (a click or Enter): a muted one does not.

## Inside `run_menu_row`: `Some(id) => self.execute_with(cx, id, linha.args.clone()),`

The ARGS come from the ROW: it was drawn knowing which panel it
speaks of, and reading the panel again here could disagree.
