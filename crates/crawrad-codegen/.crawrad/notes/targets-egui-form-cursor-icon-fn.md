---
id: targets-egui-form-cursor-icon-fn
kind: doc
target_file: src/targets/egui/form.rs
line_hint: 836
author: 
created: 
title: "CURSOR_ICON_FN"
tags: []
signature: "const CURSOR_ICON_FN : & str = \"\n/// The egui pointer of a `Cursor` (#667).\nfn cursor_icon_of(cursor: Cursor) -> egui::CursorIcon {\n    match cursor {\n        Cursor::Default | Cursor::Arrow => egui::CursorIcon::Default,\n        Cursor::Hand => egui::CursorIcon::PointingHand,\n        Cursor::Text => egui::CursorIcon::Text,\n        Cursor::Crosshair => egui::CursorIcon::Crosshair,\n        Cursor::Move => egui::CursorIcon::Move,\n        Cursor::Wait => egui::CursorIcon::Wait,\n        Cursor::NotAllowed => egui::CursorIcon::NotAllowed,\n        Cursor::ResizeNs => egui::CursorIcon::ResizeVertical,\n        Cursor::ResizeEw => egui::CursorIcon::ResizeHorizontal,\n        Cursor::ResizeNesw => egui::CursorIcon::ResizeNeSw,\n        Cursor::ResizeNwse => egui::CursorIcon::ResizeNwSe,\n    }\n}\n\""
---

The egui pointer conversion function for Cursor values.

The egui pointer of each `Cursor` (#667), written once into a form that
shows one. `Arrow` is egui's default arrow.
