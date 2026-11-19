---
id: ide-app-chrome-ideapp-escrever-linha-com-icone
kind: doc
target_file: src/ide_app/chrome.rs
line_hint: 180
author: 
created: 
title: "IdeApp::escrever_linha_com_icone"
tags: []
signature: "pub (super) fn escrever_linha_com_icone (cx : & mut Cx , item : & WidgetRef , kind : Option < f64 > , count : f64 , nome : & str , classe : & str , strip : & Texture ,)"
---

Writes a list row with an icon (Toolbox and Structure).

The `kind` is ALWAYS written, even when there is no icon, and for a
measured reason: it is a shader `instance` and its default value is
`0.0`, which is an actual drawing (the first in the list). Without writing it,
every class without an icon would render with the `ActionList` icon.

[`NO_ICON`] keeps the box RESERVED and empty, instead of hiding the
widget: hidden, the name would jump to the left and the name column
would stop aligning on the first class without a drawing.

The `count` parameter specifies the actual number of cells in the icon
strip texture, overriding the default theme token on a per-row basis.
