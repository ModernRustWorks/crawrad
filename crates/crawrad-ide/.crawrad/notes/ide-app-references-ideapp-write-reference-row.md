---
id: ide-app-references-ideapp-write-reference-row
kind: doc
target_file: src/ide_app/references.rs
line_hint: 153
author: 
created: 
title: "IdeApp::write_reference_row"
tags: []
signature: "pub (super) fn write_reference_row (cx : & mut Cx , item : & WidgetRef , row : & Row)"
---

Renders a single row in the references results list widget.

One row of the list: a file (▾/▸, name, folder, count, its mark) or
an occurrence (line number, the line with the name apart).
