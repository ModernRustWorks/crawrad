---
id: targets-egui-stubs-e-marca-do-handler
kind: doc
target_file: src/targets/egui/stubs.rs
line_hint: 73
author: 
created: 
title: "e_marca_do_handler"
tags: []
signature: "fn e_marca_do_handler (linha : & str , nome : & str) -> bool"
---

Checks whether the line is an event marker for our handler.

Is the `fn` right below this marker our stub, or a human-written method?

The question is asked by re-deriving the name from the marker, and there are **two**
accepted derivations — today's and the frozen one from before 16/09/2026
([`crate::handler_padrao_legado`]). This is not laxity: the rule is *"I only touch
what I wrote, under the name I gave it"*, and in a project generated before
that date the name we gave it was that one. Without the second, the empty stub
of every already existing project would remain in the file forever.

Recognizing is not deleting: whatever decides the pruning still requires an empty body,
no references outside the block, and nothing written by a human on top
([`podaveis`]).
