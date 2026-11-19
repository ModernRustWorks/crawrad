---
id: ide-app-actions-menu-ideapp-handle-menu-actions
kind: doc
target_file: src/ide_app/actions/menu.rs
line_hint: 7
author: 
created: 
title: "IdeApp::handle_menu_actions"
tags: []
signature: "pub (in super :: super) fn handle_menu_actions (& mut self , cx : & mut Cx , actions : & Actions)"
---

Answers the title bar, the menu bar and the open menus of a frame.

## Inside `handle_menu_actions`: `if self.ui.button(id!(top_titlebar.btn_min)).clicked(actions) {`

THE THREE WINDOW BUTTONS — minimize, maximize/restore, close.

# Why they need wiring NOW

Until this round, the ones from Makepad's `caption_bar` responded, and
ours were DRAWING: three `Button`s with their own shader and nobody
listening. With the duplicate bar removed (see `LiveHook`'s `after_apply`),
the only ones that worked went away with it — and three painted buttons
that do nothing is exactly the family of defect that `Preview` just paid for.

# Why `Window` cannot simply be called and asked

`Window` in `makepad-widgets` keeps `WindowHandle` in a PRIVATE field
(`window.rs:120`), and the public `WindowRef` exposes `resize`,
`reposition`, `get_position`, and `is_fullscreen` — nothing about minimizing,
maximizing, or closing (`window.rs:285-317`).

The path used here is the same door that `WindowHandle` uses internally,
and it is PUBLIC: `Cx::push_unique_platform_op` with a `CxOsOp`
(`makepad-platform-1.0.0/src/os/windows/win32_app.rs:197-247` does
exactly this). It is not a fork and not a drawing hack — it is the
same system command queue, called from outside.

## Inside `handle_menu_actions`: `for (lista, sub) in [`

THE CLICKED MENU ITEM.

The action comes from the SAME list that drew the row, via the index that
`PortalList` returns — no action id is written in this shell.
A DISABLED item does not run: `Action::habilitada` already said no, and
here it does not even reach `execute_with`, which would answer
"… is not available now" for an item that the screen already displayed
as disabled.
The SUBMENU (#558) is the same panel with its list: the same loop.

## Inside `handle_menu_actions`: `let max = self.ui.button(id!(top_titlebar.btn_max)).clicked(actions);`

BOTH ARE THE SAME BUTTON: one appears at a time, and the
`maximizar`/`restaurar` pair is what `CrawradRestoreButton` exists to
express. Without this, the maximized window continued showing the maximize
square — a button drawing the opposite of what it does.

## Inside `handle_menu_actions`: `let mut disparar = None;`

THE TOOLBAR COMMAND STRIP.

No action id is written here: each button knows its own via
`toolbar::Command::action`, and it is the same enum that wrote the icon.
Previously, the four ids were four loose literals in an `else if` — and
renaming the action in the catalog left the highlighted button responding
`unknown action`.

## Inside `handle_menu_actions`: `let mut alternar = None;`

THE MENU BAR: clicking the title opens its menu, and clicking the
title of an OPEN menu closes it — which is what every menu bar does.

The `Menu` comes from the SAME list that wrote the title, not from six
`if`s with handwritten widget paths.

## Inside `handle_menu_actions`: `self.reabrir_no_canvas(cx);`

Always reopen: `Undo` and `Redo` replace the entire document, and the
cost of a sweep is less than that of an `if` per command that grows
stale when the strip gains a fifth one.

## Inside `handle_menu_actions`: `if self.has_anything_to_save() {`

**DOES NOT CLOSE OVER UNSAVED WORK.** Until 17/09/2026
this shell closed directly; the egui shell has always asked, and
the question is the same — see [`IdeApp::draw_quit`].

## Inside `handle_menu_actions`: `self.toggle_maximized(cx);`

THE PAIR toggles back and forth — see the `maximizada` field. The framework's
button only went one way, and its `MaximizeWindow` covered the taskbar:
the reason and the calculation are in `ide_app/maximize.rs`.
