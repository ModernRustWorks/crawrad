---
id: ide-app-arrangement-ideapp-gravar-arranjo-se-mudou
kind: doc
target_file: src/ide_app/arrangement.rs
line_hint: 456
author: 
created: 
title: "IdeApp::gravar_arranjo_se_mudou"
tags: []
signature: "pub (super) fn gravar_arranjo_se_mudou (& mut self , cx : & mut Cx)"
---

Saves the layout to disk **when the `Dock` says it changed**.

The framework is the one that knows: `check_and_clear_need_save` (`dock.rs:1336`)
returns `true` once per change — drag, splitter moved, tab
selected, tab closed. Without it, the shell would have to compare the
entire layout on every frame, or write to disk on every frame.

## Inside `gravar_arranjo_se_mudou`: `let _ = std::fs::write(caminho, layout::encode(&arranjo));`

A write failure is SILENT here, and deliberately so: a full disk or
a read-only profile cannot become a status bar message with every
dragged splitter. What is lost is the layout memory, not the
work.
