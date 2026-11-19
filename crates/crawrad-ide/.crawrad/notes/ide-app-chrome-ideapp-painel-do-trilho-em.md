---
id: ide-app-chrome-ideapp-painel-do-trilho-em
kind: doc
target_file: src/ide_app/chrome.rs
line_hint: 572
author: 
created: 
title: "IdeApp::painel_do_trilho_em"
tags: []
signature: "pub (super) fn painel_do_trilho_em (& self , p : DVec2) -> Option < Panel >"
---

The rail panel under this point, if there is one.

It is the context menu's hit-test, and it uses the rectangles MEASURED in the
last draw — the same ones that position the menu panel. That way, what receives
the click and what opens the menu cannot disagree.
