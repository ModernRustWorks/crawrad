---
id: ide-app-constants-menu-row-height
kind: doc
target_file: src/ide_app/constants.rs
line_hint: 109
author: 
created: 
title: "MENU_ROW_HEIGHT"
tags: []
signature: "pub (super) const MENU_ROW_HEIGHT : f64 = 22.0"
---

The height of one menu row — the `CRAWRAD_MENU_LINHA` of the theme.

It is here as well as in the DSL because the popup height is arithmetic
(`rows * height + padding`) and the DSL has no arithmetic over a count it
does not know. The two are held together by
`the_menu_arithmetic_uses_the_theme_numbers`, the same arrangement every other
number shared between `tema.rs` and `ide_app/tema.rs` lives under.
