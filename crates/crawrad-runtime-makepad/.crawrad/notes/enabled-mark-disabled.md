---
id: enabled-mark-disabled
kind: doc
target_file: src/enabled.rs
line_hint: 22
author: 
created: 
title: "mark_disabled"
tags: []
signature: "fn mark_disabled () -> Vec4"
---

Returns the colour Makepad paints a disabled tick or dot with.

`THEME_COLOR_MARK_DISABLED` = `THEME_COLOR_U_2` =
`mix(#FFFFFFFF, #FFFFFF00, pow(0.85, THEME_COLOR_CONTRAST))` with the
contrast at 1.0 (`theme_desktop_dark.rs:8`, `:50-51`, `:61`, `:273`).
