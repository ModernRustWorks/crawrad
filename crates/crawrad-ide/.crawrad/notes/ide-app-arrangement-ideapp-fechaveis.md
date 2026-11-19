---
id: ide-app-arrangement-ideapp-fechaveis
kind: doc
target_file: src/ide_app/arrangement.rs
line_hint: 375
author: 
created: 
title: "IdeApp::FECHAVEIS"
tags: []
signature: "pub (super) const FECHAVEIS : & 'static [Panel] = & [Panel :: Explorer , Panel :: Components , Panel :: Inspector , Panel :: Packages , Panel :: Chat , Panel :: Debug , Panel :: References , Panel :: Problems , Panel :: Extensions , Panel :: Notes ,]"
---

The panels that the user can CLOSE via the header X.

All except `Design`: closing the form is not arranging the window, it is
dismissing the document — and for that there is `File ▸ Close`, which goes through
history and asks whatever needs to be asked. An X that closed the center
would leave the IDE without what it exists to show, due to a 20 px click.
