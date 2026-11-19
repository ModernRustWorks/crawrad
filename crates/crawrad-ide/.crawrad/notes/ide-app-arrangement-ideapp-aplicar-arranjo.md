---
id: ide-app-arrangement-ideapp-aplicar-arranjo
kind: doc
target_file: src/ide_app/arrangement.rs
line_hint: 55
author: 
created: 
title: "IdeApp::aplicar_arranjo"
tags: []
signature: "pub (super) fn aplicar_arranjo (& mut self , cx : & mut Cx , comando : crawrad_actions :: LayoutCommand)"
---

Executes a layout command — **the single door**.

The rail, the menu, and the MCP all arrive here, because they all pass through
the same catalog action (ADR 0014, Amendment 10: *"the command comes first, and
dragging is a second door for it — never a parallel path with its own
logic"*).

And dragging also arrives here, through a path that is literally the
same: [`IdeApp::mover_painel`] calculates a point inside the dock and calls
`DockRef::drop_move` (`dock.rs:1258`), which is the function that the drag `Drop`
calls. There is no splitter arithmetic written in this codebase.
Every layout command ends up REWRITING panel chrome.

# The third time of the same defect, which is why it is now handled here

`Dock` recreates an item's widget when the layout changes, and whatever was
written in Rust to that widget dies with it — **without any error**. It already
happened with `DesignCanvas` (it drew an empty document), with the
`Properties | Events` strip (it came out without text), and now with the panel
TITLE and the `X` glyph, which disappeared when switching panels via the rail.

Handling it in `load_state` was not enough: showing, hiding, moving, stacking, and
dropping also recreate items. The right place is the SINGLE DOOR through which every
layout command passes — this function —, not each of its branches.
