---
id: explorer-rows-row-explorerrow-note
kind: doc
target_file: src/explorer_rows/row.rs
line_hint: 91
author: 
created: 
title: "ExplorerRow::note"
tags: []
signature: "fn note (& self) -> Option < String >"
---

Message for the status bar when the row has additional context.

What the status bar says after the click, when the row knows something
the action does not.

# Why it is a slot of its own and not a fifth `Activation`

The file row has to do TWO things with one click: open the file — which
belongs to the catalogue, so an agent can do it too — and say **who
owns the file**, which the catalogue cannot know and which is the most
expensive information on this screen (ADR 0016). This shell has no
tooltip, so a click is the only carrier the sentence has.

A variant carrying an action AND a sentence would make every other row
answer a question it does not have, and would put the two concerns in
one value. A second slot with a default of `None` costs the rows that
have nothing to add exactly nothing.
