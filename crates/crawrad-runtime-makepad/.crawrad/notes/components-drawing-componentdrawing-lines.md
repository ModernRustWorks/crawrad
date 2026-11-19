---
id: components-drawing-componentdrawing-lines
kind: doc
target_file: src/components/drawing.rs
line_hint: 92
author: 
created: 
title: "ComponentDrawing::lines"
tags: []
signature: "fn lines (& self , _c : & MakepadCtl) -> Vec < ContentLine >"
---

WHERE this component's content in lines comes from.

Pure on purpose: a document with an N-line `Memo` has to produce N lines
here, and no `Cx` is needed to demand that.
