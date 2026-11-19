---
id: ide-app-host-api-ideapp-trabalho-nao-salvo
kind: doc
target_file: src/ide_app/host_api.rs
line_hint: 57
author: 
created: 
title: "IdeApp::TRABALHO_NAO_SALVO"
tags: []
signature: "pub (super) const TRABALHO_NAO_SALVO : & 'static str = \"the open form has unsaved changes — save them or undo them first\""
---

Rejection phrase when there are unsaved changes in the form.

The rejection for the THREE gestures that drop the open form — opening another,
reloading, and closing.

One phrase for all three because it states the FACT (there is unsaved work),
and not the gesture: naming the gesture would force one phrase per caller, and the
first to grow stale would be that of the caller born later — which is what
happened with the *"before opening another"* that was here when
`Reload` was wired.
