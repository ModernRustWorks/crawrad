---
id: targets-egui-form-filhos-desenhaveis
kind: doc
target_file: src/targets/egui/form.rs
line_hint: 669
author: 
created: 
title: "filhos_desenhaveis"
tags: []
signature: "fn filhos_desenhaveis < 'a > (doc : & Document , n : & Node , emitidos : & [& 'a Node]) -> Vec < & 'a Node >"
---

The DIRECT children of `n` that this target draws, in document order.

The list of emitted nodes is the yardstick of who appears — it has already filtered out what does not go
to the screen —, and that is why intersection is used instead of reading raw `n.children`.
