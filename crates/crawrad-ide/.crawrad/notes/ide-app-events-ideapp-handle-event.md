---
id: ide-app-events-ideapp-handle-event
kind: doc
target_file: src/ide_app/events.rs
line_hint: 1
author:
created:
title: "IdeApp::handle_event"
tags: []
signature: "fn handle_event (& mut self , cx : & mut Cx , event : & Event)"
---

Every event of the window passes here first, before the widget tree.

## Inside `handle_event`: `if md.button.is_primary() {`

RIGHT-CLICKING over a rail button opens the panel menu.

It is handled here, in the raw event, and not on the `Button`: the
`Button` of Makepad only emits clicks for the PRIMARY button
(`button.rs:674`), so a `clicked()` would never see the right button.
The hit-test uses the rectangles that the drawing measured — see
`rail_rects`.
And over the BODY of a panel, too — which is the doorway of `Design`
and `Chat`, the two that do not have a button on the rail.

The rail comes first because its button sits ON TOP of the body of
whatever is underneath; without this order, the menu would refer to
the wrong panel in the left column.
The LINK, and it arrives here via the PRIMARY button: two clicks on
a row of the `Events` tab name the handler and lead to the code — the
gesture of the classic Object Inspector.

In the raw event for the same reason as the right button, and for one
more: neither the `Button` nor the `TextInput` of Makepad counts taps,
and the only one that counts — the `Hit` — would require a click target
over the value field, which then could no longer be edited. It is
written in `rects_dos_eventos`.

## Inside `handle_event`: `self.tool_armed = None;`

**DISARMS.** The same `pressed` that began this drag also
armed the tool — and it had to arm it, because `clicked` never
happens (see the comment in `handle_actions`). Without this line,
dragging would create the component and leave the weapon loaded:
the next click on the form would create a second one that nobody
asked for.

## Inside `handle_event`: `let ponto = canvas.borrow::<DesignCanvas>().map(|c| {`

WITHOUT a transformation there is nowhere to drop, and inventing
an identity would place the component in the wrong corner
without warning. If the canvas is not there, the gesture
simply did not happen.

## Inside `handle_event`: `let canvas = self.canvas();`

CLICK OUTSIDE CLOSES — and the test is done BEFORE `ui` sees
the event, because what closes the menu is the click's existence,
not the underlying widget responding to it.

The menu bar is counted together with the panel: without this,
clicking `Edit` with `File` open would close here and reopen in
`handle_actions`, and the menu would flicker without switching.
Clicking INSIDE the panel also does not close here — what closes
it is the clicked item, after executing.
THE PALETTE DROP ON THE FORM. The shell handles it, not the canvas:
what is dropped is a palette CLASS, and only the shell has the
catalogue. Click-to-arm and drag-to-drop are two gestures of the
classic RAD IDE; both start on the same `FingerDown` (the Windows
`DoDragDrop` is synchronous and eats the button's `clicked`), so the
drop creates here directly and DISARMS the tool. Window pixels become
design units through the canvas `Transformacao`, the adapter's only
ruler (ADR 0019).

## Inside `handle_event`: `if self.tecla_do_editor_de_texto(cx, event) {`

**THE KEYS THAT THE FRAMEWORK LEFT DEAD** — `Ctrl+←/→`,
`Ctrl+Home/End`, and `Ctrl+Y`. Before `ui` for the same reason as
the line above: `CodeEditor` matches these chords in an EMPTY arm,
and letting them pass through would execute both our calculation and
its calculation on the same press. See [`editor_keys`].

## Inside `handle_event`: `if self.menu_keys(cx, event) || self.enter_no_editor_somente_leitur...`

`Event::Draw` does NOT go to `ui`: see the note on `handle_draw_2d`.
**`Enter` DOES NOT REACH THE READ-ONLY EDITOR**, and the guard is
ours because the framework's guard is missing.

The owner saw the damage: *"in editor only enter works"* — and what
it did was BREAK the line of a file that the bar swears is `Read only`.

Measured: in `CodeEditor` every arm that mutates text checks
`if !self.read_only` — typing (`code_editor.rs:845`), deleting
(`:874`), `Tab` (`:872`), cut (`:912`), paste (`:922`). **The
`ReturnKey` arm (`:861-869`) does not check**, and calls
`session.enter()` regardless. It is a flaw in the framework, and this
shell's `CodeView` is born with `read_only: true` (`code_view.rs:22`).

It cannot be fixed there, so the key is swallowed here — and ONLY
when the editor has focus, otherwise a dialog's `Enter` would die
along with it.

## Inside `handle_event`: `if let Event::MouseMove(mm) = event {`

**THE POINTER RESETS TO AN ARROW ON EVERY MOVEMENT**, and whoever
wants another cursor requests it afterwards.

# The defect, and it is in the framework

Makepad's `Splitter` requests the resize cursor on `FingerHoverIn`
and **does not return it on `FingerHoverOut`**: that branch only
touches the animator (`makepad-widgets-1.0.0/src/splitter.rs:196-205`).
Since `Cx::set_cursor` is last-caller-wins
(`makepad-platform-1.0.0/src/cx_api.rs:320-329`), the resize cursor
remained on screen until someone asked for another.

The owner described exactly this: *"when moving the mouse to the
Inspector area the cursor does not return to an arrow, it continues
with the image shape; only when I go to the editor or a button"* —
because the editor and the button request a cursor, and the empty
panel requests nothing.

# Why here, BEFORE `ui`

This is the only point through which every movement passes before
the widget tree sees the event. Requesting the arrow here provides
the default to everyone; whatever has an opinion — the splitter,
the canvas, the editor — overwrites right after, and wins. The
opposite (clearing at the end) would erase the request of whoever
just made it.

## Inside `handle_event`: `if let Event::WindowDragQuery(dq) = event {`

**WINDOW DRAGGING BY THE TITLE BAR** (WindowDragQuery).

On Windows, Makepad dispatches `Event::WindowDragQuery` on WM_NCHITTEST
to determine if the mouse position corresponds to HTCAPTION. Since
the shell draws its own bar (`top_titlebar`) and hides Makepad's
`caption_bar`, the framework's `Window` ignored the query
(`visible == false`). We answer here to allow smooth native window
dragging, double-click to maximize/restore, and the Windows system menu.

## Inside `handle_event`: `self.warm_up_list_rows(cx);`

The rows of the Components lists are born now, so their
shaders compile with the startup and not on the first click
of the rail (#266).

## Inside `handle_event`: `self.montar_cromo(cx);`

The CHROME enters the same safety net, and for the same reason:
if `Event::Startup` ran before the widget tree existed, the tab
labels and the gear icon would remain blank forever — blank is how
the DSL declares them.

## Inside `handle_event`: `if self.status.is_none() {`

**THE REJECTED PACKAGE SPEAKS AT STARTUP.**

Without this, someone who installed a component that didn't pass
opens the IDE, looks at the palette, sees nothing, and has nowhere
to begin. The loading gate has always collected one phrase per
rejection; until 2026-09-18 no one displayed them.

It does not overwrite the status if one already exists: the
warning is from startup, and startup hasn't produced any message yet.

## Inside `handle_event`: `self.fit_on_screen_once(cx);`

The startup safety net: if `Event::Startup` ran before the widget
tree existed, the first subsequent event rewrites what is derived
from the document. It costs a comparison of two Strings.
THE WINDOW FITS ON THE SCREEN, and that is measured only once —
see [`IdeApp::fit_on_screen_once`]. Here, and not in
`Event::Startup`, because there the window geometry might not
exist yet.

## Inside `handle_event`: `let escalado = self.com_ponteiro_na_escala(cx, event);`

THE POINTER ENTERS THE DRAWING SCALE, and this is the first line
because **everything** below it measures position: the canvas
gesture, the splitter, the menu that closes when clicking outside,
and the widget tree.

The framework does this automatically on macOS and does not on
Windows — the measurement is in the header of [`ui_zoom`]. With
the interface at 150% and without this line, the IDE draws large
and every click lands where the 100% drawing used to be.

It returns `None` when there is nothing to correct, which is the
common case — zoom at 100%, or an event without position —, and
in that case there is no copy at all.

## Inside `handle_event`: `let welcome = id!(designer_row.welcome_container);`

The Welcome page chosen at startup, once the dock has made the
centre's widgets: until then setting their visibility is lost (#555).

## Inside `handle_event`: `DragHit::Drag(f) if classe_arrastada(&f.items).is_some() => {`

`Copy` and not `Move`: the palette does NOT LOSE the component
when dragging. The system cursor indicates this.

## Inside `handle_event`: `if let Some(pouso) = self.pouso_pendente {`

THE CURSOR LANDING, now that the editor has drawn and has an
area — see [`IdeApp::pousar_cursor`].

## Inside `handle_event`: `let row = self`

A row names its own target; blank space below the rows is the
project's root, as a folder.
