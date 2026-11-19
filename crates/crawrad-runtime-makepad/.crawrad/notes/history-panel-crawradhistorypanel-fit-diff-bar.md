---
id: history-panel-crawradhistorypanel-fit-diff-bar
kind: doc
target_file: src/history_panel.rs
line_hint: 505
author: 
created: 
title: "CrawradHistoryPanel::fit_diff_bar"
tags: []
signature: "fn fit_diff_bar (& mut self , cx : & mut Cx2d)"
---

Keeps `Revert to this version` whole when the diff bar is narrow.

The bar is a row of fixed widths, and the counter on its left grows
from `No changes` to `3 changes · block 1 of 3` once a version with
changes is chosen: on a narrow centre the last button was cut at the
panel's edge. The legend is the one piece that only explains, so it
is what leaves; it comes back when the bar has room for it again. The
rects are the frame just drawn, so a change shows on the next one —
asked for with a `NextFrame`: a `redraw` requested while drawing is
dropped, and the bar stayed cut until something else redrew it.
