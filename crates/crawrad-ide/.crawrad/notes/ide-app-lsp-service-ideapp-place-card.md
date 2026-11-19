---
id: ide-app-lsp-service-ideapp-place-card
kind: doc
target_file: src/ide_app/lsp_service.rs
line_hint: 516
author: 
created: 
title: "IdeApp::place_card"
tags: []
signature: "pub (super) fn place_card (& mut self , cx : & mut Cx , card : & ViewRef , anchor : crawrad_host :: hover_card :: Anchor , size : (f64 , f64) ,)"
---

Places a floating card widget relative to an anchor inside the window.

A card of `size` shown at `anchor`: above the line when it fits, never
out of the window (`hover_card::place`). The hover and the parameter
card both use it.
