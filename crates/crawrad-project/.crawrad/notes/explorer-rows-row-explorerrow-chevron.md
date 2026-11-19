---
id: explorer-rows-row-explorerrow-chevron
kind: doc
target_file: src/explorer_rows/row.rs
line_hint: 76
author: 
created: 
title: "ExplorerRow::chevron"
tags: []
signature: "fn chevron (& self) -> Option < RowIcon >"
---

The chevron in the GUTTER, for a row that folds.

A slot of its own and not a second value of [`Self::icon`], because the
two are different columns: a member node shows a chevron **and** a
folder, exactly as the reference does. The gutter is the same width on
every row of the family, so a file lines up under the folder that
contains it even though only the folder has a chevron in it.
