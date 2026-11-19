---
id: ide-app-document-flow-ideapp-window-id
kind: doc
target_file: src/ide_app/document_flow.rs
line_hint: 52
author: 
created: 
title: "IdeApp::window_id"
tags: []
signature: "pub (super) fn window_id (& self , cx : & Cx) -> makepad_widgets :: WindowId"
---

The canvas widget — the only gate between the shell and drawing.

It comes via the DOCK ([`IdeApp::painel`]), and never through `self.ui`: the
window path is cached and points to the `DesignCanvas` that the
`Dock` already threw away. That is what erased the form from the screen — the
measurement is in the doc for [`IdeApp::painel`].
The window id of this IDE, for system window commands.

By POSITION, and not via `CxWindowPool::id_zero()`: `id_zero` returns
generation 0, and generation is what distinguishes a live window from a
reused slot in the pool. `window_id_contains` returns the real generation, and
falls back to index 0 if not found — which is the same destination, without the lie.

The point is the window origin plus one pixel: the internal comparison is
inclusive on both edges, and the exact origin would also work — the extra
pixel just makes the intent readable.
