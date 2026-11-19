---
id: ide-app-arrangement-ideapp-warm-up-list-rows
kind: doc
target_file: src/ide_app/arrangement.rs
line_hint: 323
author: 
created: 
title: "IdeApp::warm_up_list_rows"
tags: []
signature: "pub (super) fn warm_up_list_rows (& mut self , cx : & mut Cx)"
---

Pre-warms list row shaders for the Components panel.

Instantiates one row of every list template of the Components panel,
so its shaders compile at startup and not on the first click (#266).

# The first-show delay this removes

Makepad compiles a draw shader the first time a widget using it is
INSTANTIATED (`makepad-platform-1.0.0/src/draw_vars.rs:609` queues it
in `compile_set`) and runs the queue after the next draw
(`os/windows/windows.rs:153` → `hlsl_compile_shaders`, `d3d11.rs:459`,
one `D3DCompile` per shader). A `PortalList` only instantiates its
rows when it draws them, so the Toolbox and Structure rows paid that
compile on the first click of the rail.

Measured on 30/09/2026 (release build, the owner's layout, a
stopwatch from the `Toggle` to the `NextFrame` after the first draw —
the table is in the PR): the first show took 1395-1752 ms to paint,
the next ones 6-9 ms. Our own draw was 8-10 ms both times — the cost was
all in the paint after it.

The rows are created at entry ids no list ever draws, so they never
reach the screen. Shaders are cached by pointer for the whole process,
so a panel the `Dock` recreates later does not compile them again.
