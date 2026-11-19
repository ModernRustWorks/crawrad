---
id: layout-normalize-bar
kind: doc
target_file: src/layout.rs
line_hint: 143
author: 
created: 
title: "normalize_bar"
tags: []
signature: "fn normalize_bar (pass : & mut Pass , id : & str)"
---

Lays out controls inside a ToolBar.

**A ToolBar lays out its controls** (#457), as the classic RAD IDE's
does: left to right, in the tree's order, centred on the bar's height. A
button (Button, SpeedButton, BitBtn) takes `ButtonWidth` x `ButtonHeight`;
any other control (an Edit, a ComboBox) keeps its own size. The box this
writes is the one both targets draw, so the generated app needs nothing
of its own.
