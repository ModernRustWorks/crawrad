---
id: ide-app-status-bar-icons
kind: doc
target_file: src/ide_app/status_bar.rs
line_hint: 66
author: 
created: 
title: "ICONS"
tags: []
signature: "pub (super) const ICONS : & [(& [LiveId] , icon :: Chrome)] = & [(id ! (bottom_status_bar . cell_problems . ic_errors) , icon :: Chrome :: Error ,) , (id ! (bottom_status_bar . cell_problems . ic_warnings) , icon :: Chrome :: Warning ,) , (id ! (bottom_status_bar . cell_target . ic_target) , icon :: Chrome :: Packages ,) , (id ! (bottom_status_bar . cell_zoom . ic_zoom) , icon :: Chrome :: Zoom) ,]"
---

The icon of every cell that has one, and where it goes.

Separate from [`SLOTS`] because it is written **once**, at startup: a glyph
does not change with the document. The text does, and paying for it every
frame is what the comparison above exists to avoid.
