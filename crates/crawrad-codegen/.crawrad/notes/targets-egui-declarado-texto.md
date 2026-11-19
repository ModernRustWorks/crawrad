---
id: targets-egui-declarado-texto
kind: doc
target_file: src/targets/egui/declarado.rs
line_hint: 211
author: 
created: 
title: "texto"
tags: []
signature: "fn texto (nome : & str , desc : & ComponentDesc , de : & TextSource , color : Ink , centered : bool ,) -> Option < String >"
---

The verb that writes.

A caption coming from a property is emitted with an emptiness check — blank text
requires no drawing at all, which is what the canvas already does.
