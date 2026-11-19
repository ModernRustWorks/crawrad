---
id: ide-app-view-state-ideapp-centro-das-abas
kind: doc
target_file: src/ide_app/view_state.rs
line_hint: 224
author: 
created: 
title: "IdeApp::centro_das_abas"
tags: []
signature: "pub (super) fn centro_das_abas (& self) -> Center"
---

Which view the center must show, given what the TABS say.

The two states go together and this is the only place that stitches them:
a file tab in front is [`Center::Code`], canvas in front is the design
mode — and which of the two modes is the question `Alt+F12` answers,
which is why `FormSource` is preserved instead of becoming `Design`.
