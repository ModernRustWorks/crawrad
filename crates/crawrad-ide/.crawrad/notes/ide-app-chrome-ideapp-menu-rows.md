---
id: ide-app-chrome-ideapp-menu-rows
kind: doc
target_file: src/ide_app/chrome.rs
line_hint: 226
author: 
created: 
title: "IdeApp::menu_rows"
tags: []
signature: "pub (super) fn menu_rows (& self) -> Vec < menu :: Row >"
---

The open menu's rows, from the catalog, evaluated for the current context.

Reassembled on every draw, and not cached: `Action::habilitada` inspects the
selection, the clipboard, and the history, and all three change with the
menu open — `Ctrl+C` in another window doesn't, but clicking on the form
behind it does. A menu with the state frozen from when it opened lights up
an item that can no longer run.

The registry is the SAME one that `designer` loads, for the reason written in
[`IdeApp::linhas_da_paleta`].
