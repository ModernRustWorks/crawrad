---
id: ide-app-explorer-actions-ideapp-raiz
kind: doc
target_file: src/ide_app/explorer_actions.rs
line_hint: 47
author: 
created: 
title: "IdeApp::raiz"
tags: []
signature: "pub (crate) fn raiz (& self) -> Option < std :: path :: PathBuf >"
---

The GROUP root — the folder from which the Explorer tree hangs.

The egui shell receives this folder from `Open Folder`; this one opens a form
by path and has no one to tell it, so it is derived —
see [`explorer::group_root_of`].
