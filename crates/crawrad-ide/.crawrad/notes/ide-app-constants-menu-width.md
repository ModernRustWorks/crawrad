---
id: ide-app-constants-menu-width
kind: doc
target_file: src/ide_app/constants.rs
line_hint: 112
author: 
created: 
title: "MENU_WIDTH"
tags: []
signature: "pub (super) const MENU_WIDTH : f64 = 260.0"
---

The fixed width of the popup — the `CRAWRAD_MENU_LARGURA` of the theme.

In Rust as well as in the DSL because the context menu is clamped to the
window, and a clamp needs the width the DSL declares.
