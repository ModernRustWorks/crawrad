---
id: ide-app-build-ideapp-bombear
kind: doc
target_file: src/ide_app/build.rs
line_hint: 10
author: 
created: 
title: "IdeApp::bombear"
tags: []
signature: "pub (super) fn bombear (& mut self , cx : & mut Cx)"
---

Drains the design canvas's pointer events since the last frame.

## Inside `bombear`: `canvas.redraw(cx);`

AND THE CANVAS, EXPLICITLY — the `self.ui.redraw` above does NOT go down
to it.

The owner described the symptom: *"you click and it takes forever for the
handle to appear on the clicked component"*. Measured with a screenshot taken right
after the click: the Inspector already showed `btn_ok · Button` with all
properties, the bar said `btn_ok selected`, and **the canvas had no
handles at all**. The chrome repainted because each `set_text` dirties its
widget; the canvas, which nobody touched, remained with the previous frame and only
repainted when another gesture woke it up.

It is the same defect as the `Preview` branch (see `AlternarVivo`), and the
same fix: the canvas is what repaints the canvas.

## Inside `bombear`: `if let RuntimeEvent::PointerDblClick { x, y } = ev {`

THE DOUBLE CLICK ON THE FORM — the reason the editor exists
(ADR 0015), and the oldest gesture in a classic RAD IDE.

It comes BEFORE `designer` and does not pass through it: the `PointerDown`
of the same touch already selected the component (the translation stacks the
two, `canvas.rs:829-836`), and what remains is the trip to code.
It needs the entire `&mut self`, which is why it is here and not in the
lower `match`, which holds the document borrowed.

## Inside `bombear`: `if let Err(e) = rt.open(d.ir()) {`

Reopening is the honest path: `Applied`'s `touched` gets
lost in the loop above. And the error goes to the bar instead of
a `let _ =` — a canvas that stops drawing without saying anything is
the family of defects that this round came to eliminate from the screen.

## Inside `bombear`: `self.status = None;`

A message is TRANSITORY: it occupies the place of the file
path in the bar, and a new gesture restores the path. Without
this, the first notice would stay there forever and the bar
would stop telling what is open.

## Inside `bombear`: `if let RuntimeEvent::PointerDown { x, y, .. } = ev {`

Two gestures are decided BEFORE the designer, and both need
the entire `&mut self`: releasing the armed palette tool and
clicking on the tab strip of a `PageControl` — which switches the
page, and does not select the component.
