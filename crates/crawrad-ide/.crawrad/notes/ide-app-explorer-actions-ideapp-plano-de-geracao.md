---
id: ide-app-explorer-actions-ideapp-plano-de-geracao
kind: doc
target_file: src/ide_app/explorer_actions.rs
line_hint: 71
author: 
created: 
title: "IdeApp::plano_de_geracao"
tags: []
signature: "pub (super) fn plano_de_geracao (& self ,) -> Option < Result < Vec < crawrad_codegen :: Arquivo > , crawrad_codegen :: Erro > >"
---

What the next `Generate` would write, **without writing anything**.

It is the single source for the Explorer, and it is **the same function that the button
calls**: [`crawrad_project::generator::plan_with_prepass`], with the same generator
(`generator::project_generator`) and the same pruning pre-pass (ADR 0025).

The opposite was once written here, and it was right back then: while this
shell lacked `Generate`, building the plan with pruning would promise an
effect that nothing here produced, and the target came directly from the
saved preference. Now the button exists (issue #147), and the two reasons
inverted: the panel must show what this shell's `Generate`
would do, otherwise it shows one plan and the button writes another — the most
expensive flaw this screen could have (ADR 0016).

The generator target is read from the process and not from the preferences: the one
that populates it, once, is [`IdeApp::handle_startup`].
