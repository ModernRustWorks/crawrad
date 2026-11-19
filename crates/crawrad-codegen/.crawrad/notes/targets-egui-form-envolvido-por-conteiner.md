---
id: targets-egui-form-envolvido-por-conteiner
kind: doc
target_file: src/targets/egui/form.rs
line_hint: 660
author: 
created: 
title: "envolvido_por_conteiner"
tags: []
signature: "fn envolvido_por_conteiner (doc : & Document , id : & str) -> bool"
---

Checks whether the node is drawn inside a wrapping container.

**Is this node drawn INSIDE a wrapping container?**

The question is about the entire lineage, and not just the parent: a `Button` inside
a `Panel` that is inside a `ScrollBox` also comes from inside the box.
