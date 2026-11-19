---
id: controller-debugcontroller-drop
kind: doc
target_file: src/controller.rs
line_hint: 201
author: 
created: 
title: "DebugController::drop"
tags: []
signature: "fn drop (& mut self)"
---

Terminates the debug session when the controller is dropped.

Leaving the IDE (or starting another session) ends this one: the
program under the debugger does not outlive the window that drove it.
