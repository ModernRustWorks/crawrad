---
id: ide-app-packages-app-ideapp-escrever-linha-de-pacote
kind: doc
target_file: src/ide_app/packages_app.rs
line_hint: 134
author: 
created: 
title: "IdeApp::escrever_linha_de_pacote"
tags: []
signature: "pub (super) fn escrever_linha_de_pacote (cx : & mut Cx , item : & WidgetRef , linha : & packages :: Row)"
---

Writes a Package Manager line into the widget created by the template.

#### At `for (slot, caixa) in PACKAGE_SLICES.iter().enumerate() {`

THE RIGHT COLUMN SLICES, and the leftover one DISAPPEARS.

Hide, and do not write empty: an empty label keeps
requesting height, and a card of a package without statistics
would have a gap the size of three lines that no one can
explain. It is the same rule as the ImageList grid alongside.

#### At `let mut dentro = caixa.to_vec();`

THE LABEL PATH IS DERIVED from the box
path, and not written alongside: two lists with the
same name written twice is the silent
divergence this panel has already paid for.

## Inside `escrever_linha_de_pacote`: `if *installed && !*can_update {`

The wording for the package the project ALREADY HAS comes from the model, and
not from the DSL: it is the SAME one that the egui window writes, and
written in both places it would diverge on the first rewrite.
