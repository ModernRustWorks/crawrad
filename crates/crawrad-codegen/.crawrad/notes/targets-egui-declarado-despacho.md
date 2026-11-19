---
id: targets-egui-declarado-despacho
kind: doc
target_file: src/targets/egui/declarado.rs
line_hint: 99
author: 
created: 
title: "despacho"
tags: []
signature: "fn despacho (nome : & str , ligacoes : & [(String , String)]) -> String"
---

Dispatch, AFTER drawing — and under the guard of `enabled`.

Disabled does not disappear, it dims: it remains drawn and **ceases to
respond**, which is what a classic RAD IDE does and what this target already does with
natives.
