---
id: ide-app-events-ideapp-handle-key-down
kind: doc
target_file: src/ide_app/events.rs
line_hint: 32
author: 
created: 
title: "IdeApp::handle_key_down"
tags: []
signature: "fn handle_key_down (& mut self , cx : & mut Cx , ke : & KeyEvent)"
---

THE KEY, when NO text field has focus.

This hook belongs to `MatchEvent` (`makepad-draw-1.0.0/src/match_event.rs:56`,
dispatched at `:86`), and it receives **every** `Event::KeyDown` — the filter
by focus is `event.hits`, which only widgets call. That is why the shell
needs to decide on its own when to stay silent, and the decision is a single
line:

With a text field in focus, the key belongs to IT first. Whatever it
rejects comes back via `TextInputAction::KeyDownUnhandled`
(`makepad-widgets-1.0.0/src/text_input.rs:1700`) and is handled in
[`IdeApp::handle_actions`]. This is the same rule as the egui shell
(`main.rs:5266`, `e_do_campo_de_texto`) — with the difference that here
**the field is what answers**, not a list of keys written on the IDE side,
which is what that comment over there claimed to want and failed to achieve.
A new chord in the catalog is born working inside the field; only the conflict
needs to be written, and it writes itself.

Confirmation with `cx.has_key_focus` (`cx_api.rs:584`) is what prevents the
shadow `campo_focado` from silencing the IDE by mistake — see the field.
