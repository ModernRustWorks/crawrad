---
id: ide-app-dialogs-imagelist-ideapp-fileiras-das-imagens
kind: doc
target_file: src/ide_app/dialogs/imagelist.rs
line_hint: 33
author: 
created: 
title: "IdeApp::fileiras_das_imagens"
tags: []
signature: "pub (in super :: super) fn fileiras_das_imagens (& self) -> usize"
---

The grid rows: the indices of each row of six.

The `PortalList` draws a LIST, and the egui grid has six columns — so
the list is made of rows, and what determines what goes into each cell is this
calculation, in a single place. See [`IML_COLUNAS`].
