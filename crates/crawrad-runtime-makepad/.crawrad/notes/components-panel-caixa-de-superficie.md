---
id: components-panel-caixa-de-superficie
kind: doc
target_file: src/components/panel.rs
line_hint: 39
author: 
created: 
title: "caixa_de_superficie"
tags: []
signature: "pub (crate) fn caixa_de_superficie (p : & mut Brush , r : Rect)"
---

The box that serves as a surface for a container.

The `ScrollBox`, `TabSheet`, and the body of `PageControl` call this same
function: they are the same figure, and repeating the pair of tokens across four files is
exactly the double maintenance point that house rules forbid.

The color is that of the CONTAINER ([`MAKEPAD_PANEL`]), and not the face of `Button`. They
used to be the same, and that is part of what the owner rejected: panel and button
indistinguishable on screen. The RXP separates the two for the same reason
(`crawrad-runtime-egui/src/lib.rs:172-179`).
