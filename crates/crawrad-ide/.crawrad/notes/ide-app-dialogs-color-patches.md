---
id: ide-app-dialogs-color-patches
kind: doc
target_file: src/ide_app/dialogs/color.rs
line_hint: 24
author: 
created: 
title: "patches"
tags: []
signature: "fn patches (recent : & [Color]) -> impl Iterator < Item = (LiveId , Option < Color >) > + '_"
---

Returns the 24 palette patches and their colors.

The 24 patches — the 16 basic colours, then the 8 recent slots — and the
colour of each, `None` for a recent slot still empty.
