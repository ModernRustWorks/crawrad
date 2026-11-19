---
id: ide-app-aberto
kind: doc
target_file: src/ide_app.rs
line_hint: 1039
author: 
created: 
title: "Aberto"
tags: []
signature: "enum Aberto"
---

Which menu is open — that of a bar title, or that of a PANEL.

Both draw in the same floating panel and are the same list of rows
([`menu::Row`]); what changes is where the rows come from and where the panel
anchors. Two separate fields would produce the invalid state "both open",
which the screen cannot draw.
