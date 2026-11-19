---
id: ide-app-arrangement-ideapp-painel-a-vista
kind: doc
target_file: src/ide_app/arrangement.rs
line_hint: 37
author: 
created: 
title: "IdeApp::painel_a_vista"
tags: []
signature: "pub (super) fn painel_a_vista (& self , arranjo : & std :: collections :: HashMap < LiveId , DockItem > , p : Panel ,) -> bool"
---

The panel that is **in view** right now, panel by panel.

Derived from the `Dock`, never stored: a panel is in view when its tab
exists and is the selected one in the group it landed in. It is this function that
lights up the rail accent bar and decides what `Toggle` does.
