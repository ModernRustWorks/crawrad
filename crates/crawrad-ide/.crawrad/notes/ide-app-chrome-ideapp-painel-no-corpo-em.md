---
id: ide-app-chrome-ideapp-painel-no-corpo-em
kind: doc
target_file: src/ide_app/chrome.rs
line_hint: 580
author: 
created: 
title: "IdeApp::painel_no_corpo_em"
tags: []
signature: "pub (super) fn painel_no_corpo_em (& self , cx : & Cx , arranjo : & std :: collections :: HashMap < LiveId , DockItem > , ponto : DVec2 ,) -> Option < Panel >"
---

The panel whose BODY is under this point, if there is one in view.

# Why the context menu needs this gateway in addition to the rail

The rail has four buttons and the dock has six panels: `rail::RAIL` is a
subset of `Panel::ALL`, and `Design` and `Chat` are excluded from it by a
decision documented there. While the context menu only opened over a rail
button, the two had **no mouse gateway whatsoever** for `Move
To`, `Hide`, and `Float` — the command existed in the catalog, responded to the
protocol, and there was no gesture that reached it.

# Why the rectangle comes from the DOCK

`self.painel(p)` is `DockRef::item`, which reads `items` on the fly
(`dock.rs:1211`). The window path would return the widget of a
DEAD instance whenever the arrangement had changed — this is the defect that the
doc of [`IdeApp::painel`] measures by pointer, and here it would appear as
a menu that opens referring to the wrong panel.

Only a panel **in view** is taken into account: a tab that is behind another in
the same group keeps the rectangle from the last frame in which it appeared, and without
this filter the click would land on it instead of the one in front.
