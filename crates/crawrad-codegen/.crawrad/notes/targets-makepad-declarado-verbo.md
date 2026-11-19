---
id: targets-makepad-declarado-verbo
kind: doc
target_file: src/targets/makepad/declarado.rs
line_hint: 74
author: 
created: 
title: "verbo"
tags: []
signature: "fn verbo (field : & str , desc : & ComponentDesc , v : & Verb , i : usize , w : f32 , h : f32 , d : & mut Declarado)"
---

A verb: the area first, the brushstroke after.

A verb that cannot be written — one that reads a property that the class does not
declare — is **skipped**, not guessed. Painting with an invented zero would be the
component lying while pretending to work.
