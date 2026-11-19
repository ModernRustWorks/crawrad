---
id: components-memo-desenho-lines
kind: doc
target_file: src/components/memo.rs
line_hint: 30
author: 
created: 
title: "Desenho::lines"
tags: []
signature: "fn lines (& self , c : & MakepadCtl) -> Vec < ContentLine >"
---

Extracts the content lines from the Memo's text.

**The `Memo` stores content in `text`, and its `items` collection is
always empty.** While the canvas sent `items` to draw, the box
came out blank — the owner saw this in the screenshot and was right. The RXP reads
`c.text.lines()` (`crawrad-runtime-egui/src/lib.rs:2601`), and it is the same here.
