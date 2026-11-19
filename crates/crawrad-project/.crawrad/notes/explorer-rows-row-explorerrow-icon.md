---
id: explorer-rows-row-explorerrow-icon
kind: doc
target_file: src/explorer_rows/row.rs
line_hint: 71
author: 
created: 
title: "ExplorerRow::icon"
tags: []
signature: "fn icon (& self) -> Option < RowIcon >"
---

The Codicons drawing at the left of the text, when it has one.

`Option` and not a default glyph: a row with nothing to draw draws
nothing, and the name still reads. A generic icon handed back in
silence would be the panel claiming to know something it does not.
