---
id: ide-app-property-ideapp-alvos-da-edicao
kind: doc
target_file: src/ide_app/property.rs
line_hint: 44
author: 
created: 
title: "IdeApp::alvos_da_edicao"
tags: []
signature: "pub (super) fn alvos_da_edicao (& self , no : & NodeId) -> Vec < NodeId >"
---

The nodes that an Inspector edit affects.

With the draft node still in the selection, the target is the ENTIRE selection — which
is what provides the "editing with N selected undoes on all N at once" that the
action's plural `nodes` exists to offer. Outside it (the user clicked
another component with the field focused), the target is only the node they
were editing.
