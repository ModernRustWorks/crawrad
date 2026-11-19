---
id: ide-app-host-api-ideapp-aplicar-zoom
kind: doc
target_file: src/ide_app/host_api.rs
line_hint: 211
author: 
created: 
title: "IdeApp::aplicar_zoom"
tags: []
signature: "pub (super) fn aplicar_zoom (& mut self , cx : & mut Cx)"
---

Hands over the zoom to the canvas runtime.

One line, and it is worth all the rest: the adapter's `Transformacao` is
`escala = fator_do_dispositivo * zoom`
(`crates/crawrad-runtime-makepad/src/lib.rs:248`), and it is the SAME
transformation that converts the click back — the pointer continues
matching the drawing at any step (ADR 0019).
