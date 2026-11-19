---
id: designer-locked-for
kind: doc
target_file: src/lib.rs
line_hint: 305
author: 
created: 
title: "Designer::locked_for"
tags: []
signature: "fn locked_for (& self , doc : & Document , id : & NodeId) -> bool"
---

Returns whether the lock prevents modifying geometry of this node.

Does the lock prevent modifying the geometry of this node?

The rule primitive. Queried by two — [`Designer::refuses`], which is the true gate,
and `gesture_permitted` right below, which only prevents a phantom drag.

The form is excluded, and the command name is the argument: the classic IDE
locks **the controls**, not the window that contains them. There the form changes
size by the border of the window itself even with the lock on; here the
form's three handles play the role of that border.

**The form exception reaches the children, and that is faithful to the classic IDE.**
Resizing the form triggers the anchors pass and the `Align` pass
(`layout::apply_anchors` and `layout::apply`), which move controls.
There `alClient` follows the window with the lock on — the lock is about
what YOU move, not about what the layout recalculates. That is why the
shell applies these two passes directly on `Doc`, without going through the gate:
the bypass is deliberate, and the test
`anchors_pass_would_be_refused_and_that_is_why_it_bypasses_the_gate`
exists so nobody "fixes" it without noticing.

What remains valid, and is the point of the command: **selecting is not
modifying**. Click, marquee, Ctrl+click and the entire Object Inspector — typed
Left and Top included — do not pass through here.
