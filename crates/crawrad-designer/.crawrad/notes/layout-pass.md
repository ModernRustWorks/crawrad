---
id: layout-pass
kind: doc
target_file: src/layout.rs
line_hint: 82
author: 
created: 
title: "Pass"
tags: []
signature: "struct Pass < 'a >"
---

What a layout pass has already decided, but the document does not know yet.

The entire pass reads the document, and the parent's new rectangle only exists
inside the [`Transaction`] until someone applies it. Without this intermediary the
grandchild was aligned against the grandfather's OLD area and only got it right on the next
turn — measured in two turns, always, for any nested
container. Two turns is not slowness: it is a spurious transaction per gesture in
undo, and a whole frame on screen with the grandchild at the wrong size.

So there is **one** place that answers "what is its rectangle now", and it
answers with what this pass has already decided.
