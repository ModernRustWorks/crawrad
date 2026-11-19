---
id: designer-reparent-dragged
kind: doc
target_file: src/lib.rs
line_hint: 843
author: 
created: 
title: "Designer::reparent_dragged"
tags: []
signature: "fn reparent_dragged (& self , doc : & Document , start : & [(NodeId , Rect)]) -> Option < Transaction >"
---

Transaction that moves what was dragged into the container under it.

`None` when nobody changed parent — which is the common case, and why
normal drag pays nothing for this to exist.

The command is **a single one** per component: parent and relative position together. In the
classic IDE `SetParent` does not touch `Left`/`Top`, and the control teleports;
here there cannot exist even for an instant a component in the new parent with
the coordinates of the old parent
(ADR 0012).
