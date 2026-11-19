---
id: targets-egui-form-ancoragem-egui
kind: doc
target_file: src/targets/egui/form.rs
line_hint: 1033
author: 
created: 
title: "ANCORAGEM_EGUI"
tags: []
signature: "const ANCORAGEM_EGUI : & str = r#\"\n/// Where a component ends up after the form is resized.\n///\n/// `origin` is the form's top-left, `pos`/`size` are what the designer stored,\n/// and `grown` is how much the form grew since design time.\nfn anchor(\n    origin: egui::Pos2,\n    pos: egui::Vec2,\n    size: egui::Vec2,\n    a: Anchors,\n    grown: egui::Vec2,\n) -> egui::Rect {\n    let (x, y, w, h) = anchored(pos.x, pos.y, size.x, size.y, a, grown.x, grown.y);\n    egui::Rect::from_min_size(\n        origin + egui::vec2(x, y),\n        egui::vec2(w.max(1.0), h.max(1.0)),\n    )\n}\n\"#"
---

The egui anchor wrapper: `Pos2`/`Vec2` at the ends, and the FLOOR.

**The floor lives here and not in the box**, and the reason is in the doc for `anchored`: the
application recalculates from design-time constants and never saves the
result, so clamping at 1.0 destroys nothing. In the IDE it would be destructive — it
sums deltas and writes them back.
