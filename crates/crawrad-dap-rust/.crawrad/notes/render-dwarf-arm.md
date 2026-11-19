---
id: render-dwarf-arm
kind: doc
target_file: src/render.rs
line_hint: 248
author: 
created: 
title: "dwarf_arm"
tags: []
signature: "fn dwarf_arm (ty : & str , all : & [Variable] , cx : & Cx < '_ >) -> Option < Vec < Variable > >"
---

Selects the active variant arm using DWARF discriminant layouts.

The active variant by the program's own DWARF: the arm whose discriminant
value is the one stored, else the arm without one (the niche's dataful arm).

Measured on 01/10/2026 on Linux, LLDB 22: the variants are numbered by
POSITION (`$variant$0` for `None`), and every `$discr$` shows the value
stored — for an `Option<String>` that is `None`, the niche
`9223372036854775808`. Matching the number against it found nothing and the
dataful arm, `Some`, was shown. `None` when there are no layouts or the
enum is not in them: the numbered rule above then decides.
