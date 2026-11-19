---
id: ide-app-key-dispatch-ideapp-enter-no-editor-somente-leitura
kind: doc
target_file: src/ide_app/key_dispatch.rs
line_hint: 54
author: 
created: 
title: "IdeApp::enter_no_editor_somente_leitura"
tags: []
signature: "pub (super) fn enter_no_editor_somente_leitura (& self , cx : & Cx , event : & Event) -> bool"
---

Checks whether the event is Enter with the read-only editor focused.

Is the event an `Enter` with the read-only editor focused?

See the call, in `handle_event`, for the measure that forces this guard
to exist. It answers `true` for the caller to SWALLOW the key.

**The guard is only valid while the view is read-only.** Since the editor
started editing ([`editor_buffer`]), swallowing `Enter` in an
editable file would be the opposite defect: the user would type a line and
would not be able to start the next one.
