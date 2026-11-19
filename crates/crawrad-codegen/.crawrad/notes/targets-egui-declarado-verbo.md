---
id: targets-egui-declarado-verbo
kind: doc
target_file: src/targets/egui/declarado.rs
line_hint: 119
author: 
created: 
title: "verbo"
tags: []
signature: "fn verbo (nome : & str , desc : & ComponentDesc , v : & Verb , p : & mut Pintura) -> Option < String >"
---

A verb: the area, followed by the stroke upon it.

`None` for a verb that cannot be written — one that reads a property that
the class does not declare. It is **skipped**, not guessed: painting with an invented
zero would mean the component is lying while appearing to work.
