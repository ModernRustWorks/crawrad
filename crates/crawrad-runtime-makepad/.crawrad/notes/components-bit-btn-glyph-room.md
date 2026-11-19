---
id: components-bit-btn-glyph-room
kind: doc
target_file: src/components/bit_btn.rs
line_hint: 29
author: 
created: 
title: "glyph_room"
tags: []
signature: "pub (crate) fn glyph_room (p : & mut Brush , c : & MakepadCtl , inner : Rect) -> Rect"
---

Calculates the text rectangle after reserving room for the glyph.

**The room of a Glyph** (#454), shared with the `SpeedButton`: while the
button has a picture, a 16-point frame at its left — where the generated
applications draw it — and the caption beside it. The canvas paints the
glyph inside the 16-point slot (aspect kept, centred — #544).
