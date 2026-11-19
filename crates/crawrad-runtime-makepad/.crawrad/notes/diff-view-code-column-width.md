---
id: diff-view-code-column-width
kind: doc
target_file: src/diff_view.rs
line_hint: 245
author: 
created: 
title: "CODE_COLUMN_WIDTH"
tags: []
signature: "const CODE_COLUMN_WIDTH : f64 = 7.16"
---

The width of one column of the code font measured on screen.

The width of one column of the code font (`THEME_FONT_CODE`, 9 pt),
measured on screen: a line of 40 characters spans 283 px. The font is
monospaced, so each side scrolls by whole columns: the text is cut at
its first shown column rather than moved, which keeps the PortalList
rows free of a per-frame `apply_over` (it breaks their drawing order).
