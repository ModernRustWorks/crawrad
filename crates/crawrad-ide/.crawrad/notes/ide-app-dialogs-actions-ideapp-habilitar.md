---
id: ide-app-dialogs-actions-ideapp-habilitar
kind: doc
target_file: src/ide_app/dialogs/actions.rs
line_hint: 264
author: 
created: 
title: "IdeApp::habilitar"
tags: []
signature: "pub (in super :: super) fn habilitar (cx : & mut Cx , botao : & WidgetRef , ligado : bool)"
---

Enables or disables a button — **BOTH halves**.

# What was wrong, and was measured on screen

`ButtonRef::set_enabled` only touches the `enabled` field, and it is consulted
in `Hit` arms (`makepad-widgets-1.0.0/src/button.rs:674-696`):
it blocks the click and **does not change a single pixel**. The dimmed look is another state, from the
ANIMATOR (`button.rs:641`, `set_disabled` → `id!(disabled.on)`), and is
what the shader reads.

Calling only the first gives the worst of both worlds: the button looks active,
the user clicks, and nothing happens — which is exactly the defect family
that this shell removed from the status bar. On 09/17/2026 the three commands
of the ActionList Editor came out lit up without any selection, measured by
the text pixel brightness: 230 on all five, when dimmed is 107.

And dimmed MUST be the animator's, never an `apply_over` of
`disabled: 1.0` (#569): each frame of the hover animation reapplies the
entire animator state, including the `disabled` track in `off`
(`disabled: 0.0`), and the dimmed button would light up under the mouse.
