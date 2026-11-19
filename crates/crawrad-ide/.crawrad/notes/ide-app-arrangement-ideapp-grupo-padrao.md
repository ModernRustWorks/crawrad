---
id: ide-app-arrangement-ideapp-grupo-padrao
kind: doc
target_file: src/ide_app/arrangement.rs
line_hint: 269
author: 
created: 
title: "IdeApp::grupo_padrao"
tags: []
signature: "pub (super) fn grupo_padrao (p : Panel) -> LiveId"
---

The tab group where a panel originates.

**This is the region DEFAULT, not its owner** (ADR 0014, Amendment 9): the
panel is born here and goes wherever the user takes it.
