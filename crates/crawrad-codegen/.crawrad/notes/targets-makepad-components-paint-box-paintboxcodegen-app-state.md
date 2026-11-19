---
id: targets-makepad-components-paint-box-paintboxcodegen-app-state
kind: doc
target_file: src/targets/makepad/components/paint_box.rs
line_hint: 69
author: 
created: 
title: "PaintBoxCodegen::app_state"
tags: []
signature: "fn app_state (& self , node : & Node , _field : & str) -> Result < Option < AppState > , crate :: Erro >"
---

Emits `use` of widget accessor only when `OnPaint` is defined.

Only the `use` of the widget's accessor (`self.ui.crawrad_paint_box(...)`),
and only when an `OnPaint` calls it: a box nobody paints leaves an
unused import otherwise (#667).
