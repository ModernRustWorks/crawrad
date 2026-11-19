---
id: ide-app-dialogs-imagelist-ideapp-caminho-da-galeria
kind: doc
target_file: src/ide_app/dialogs/imagelist.rs
line_hint: 28
author: 
created: 
title: "IdeApp::caminho_da_galeria"
tags: []
signature: "pub (in super :: super) fn caminho_da_galeria () -> Option < std :: path :: PathBuf >"
---

Where the user's gallery lives, when this machine has a place for it.

The folder is the SAME as the dock layout ([`layout::config_dir`]): it is
an IDE configuration, not a project one — an added icon serves
all forms, which is what *gallery* means.

`None` is normal and costs nothing: the factory gallery remains intact
and the window opens. What the IDE does not do is invent a write location on
someone's disk — the same rule as for recent folders.
