---
id: targets-egui-components-emitir-de-terceiro
kind: doc
target_file: src/targets/egui/components/mod.rs
line_hint: 216
author: 
created: 
title: "emitir_de_terceiro"
tags: []
signature: "fn emitir_de_terceiro (ctx : & ContextoEmissao , classe : & str , modulo : & str , desc : & crawrad_registry :: ComponentDesc ,) -> ResultadoComponente"
---

The third-party component becomes code.

**The THIRD-PARTY component becomes code.**

A class without a handler in this target, but with a declared [`crate_module`], is not
unknown: it is a type living in a crate, and the generated application
writes it in full — `portadb_crawrad::Connection`.

It is step §4 of ADR 0033,
and the two verbs it uses are those from
ADR 0036:
`Create` becomes `Tipo::default()` and each `Set` becomes a field assignment.

# And it DRAWS, since 09/18/2026

For one day it did not draw, and the user project warning stated this out
loud: *"unused variable: `r`"* — the generator calculated the component's
rectangle and discarded it. The declared drawing worked on the IDE's canvas and
did not carry over to the target, which is worse than not supporting it: the designer
showed one thing and the application showed another.

What translates the verbs into egui painting is [`crate::targets::egui::declarado`]. The
**non-visual** component continues without any drawing, and now without a rectangle
either — what answers this is the descriptor's `visual`, not a list of
native names.

[`crate_module`]: crawrad_registry::ComponentDesc::crate_module
