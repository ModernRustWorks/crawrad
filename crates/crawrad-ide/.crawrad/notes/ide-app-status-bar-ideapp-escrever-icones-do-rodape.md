---
id: ide-app-status-bar-ideapp-escrever-icones-do-rodape
kind: doc
target_file: src/ide_app/status_bar.rs
line_hint: 168
author: 
created: 
title: "IdeApp::escrever_icones_do_rodape"
tags: []
signature: "pub (super) fn escrever_icones_do_rodape (& mut self , cx : & mut Cx)"
---

Writes the glyph of every cell that has one — once, at startup.

In Rust and not in the DSL for the reason ADR 0026 gives: the code
point lives in one place, and that place is [`icon::Chrome`].
