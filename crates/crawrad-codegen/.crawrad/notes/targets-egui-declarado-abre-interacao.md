---
id: targets-egui-declarado-abre-interacao
kind: doc
target_file: src/targets/egui/declarado.rs
line_hint: 81
author: 
created: 
title: "abre_interacao"
tags: []
signature: "fn abre_interacao (nome : & str , ligacoes : & [(String , String)]) -> String"
---

Generates the interaction area before drawing.

**THE CLICK AREA, and it comes BEFORE drawing.**

`ui.interact` reserves the rectangle before anyone paints on it — it is the same
order that this target's `CheckBox` and `Button` already follow, and it matters: painting
afterwards paints on top of what `egui` already knows is clickable.

The surface of a declared component is the **rectangle itself**, and nothing else.
It is not a toolkit widget — it is a drawing —, so there is no sub-part to
consult.
