---
id: ide-app-dialogs-actions-ideapp-mover-acao
kind: doc
target_file: src/ide_app/dialogs/actions.rs
line_hint: 184
author: 
created: 
title: "IdeApp::mover_acao"
tags: []
signature: "pub (in super :: super) fn mover_acao (& mut self , cx : & mut Cx , lista : & NodeId , passo : i32)"
---

Moves the chosen action one position within the list.

**Through the catalog action**, and this changed on 09/17/2026: it was a
`Transaction` assembled here — as it still was in the egui shell —, and the guard
`a_casca_ainda_muta_o_documento_por_fora_do_catalogo` caught the fifth
mutation without an id. It was right: reordering is a capability, and a
capability that the shell solves on its own is a capability that never reaches the
protocol (ADR 0005).

What rejects the boundary — the first with `Move Up`, the last with `Move
Down` — is the action, and not this method: it already has to find the position to
reorder, and a second calculation here would disagree with its calculation on the day
the list changed between the two.
