---
id: panel-command
kind: doc
target_file: src/lib.rs
line_hint: 4315
author: 
created: 
title: "panel_command"
tags: []
signature: "fn panel_command (c : & Context , make : fn (Panel) -> LayoutCommand) -> Effect"
---

An action that does one thing with ONE panel.

The four (`show`, `hide`, `toggle`, `float`) only differ in the command they
return, and `execute` is a function pointer **without capture** — that is why the
common body lives here and each one passes its constructor.
