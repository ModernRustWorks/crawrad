---
id: targets-egui-control-events-pointer-events
kind: doc
target_file: src/targets/egui/control_events.rs
line_hint: 27
author: 
created: 
title: "POINTER_EVENTS"
tags: []
signature: "const POINTER_EVENTS : [(& str , & str) ; 9] = [(\"on_mouse_activate\" , \"Activate\") , (\"on_mouse_down\" , \"Down\") , (\"on_mouse_up\" , \"Up\") , (\"on_mouse_move\" , \"Move\") , (\"on_mouse_enter\" , \"Enter\") , (\"on_mouse_leave\" , \"Leave\") , (\"on_click\" , \"Click\") , (\"on_dbl_click\" , \"DblClick\") , (\"on_mouse_wheel\" , \"Wheel\") ,]"
---

The pointer events delivered by the router and their mapped names.

The pointer events the router delivers, and its name for each. The
activation comes first: classic RAD fires it on the press, before the
press itself.
