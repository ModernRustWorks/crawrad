---
id: targets-egui-stubs-fecha-o-que-veio-antes
kind: doc
target_file: src/targets/egui/stubs.rs
line_hint: 260
author: 
created: 
title: "fecha_o_que_veio_antes"
tags: []
signature: "fn fecha_o_que_veio_antes (acima : & str) -> bool"
---

Whether the line just before closes a valid syntactic structure.

# Coupling with `elo::mascarar`

This check assumes uncommented code. When the user wraps a
handler in a block comment (`/* ... */`) or multiline string, what rejects
the pruning is not this function, but rather `elo::handlers_do_arquivo` operating on the
source masked by `elo::mascarar`.
