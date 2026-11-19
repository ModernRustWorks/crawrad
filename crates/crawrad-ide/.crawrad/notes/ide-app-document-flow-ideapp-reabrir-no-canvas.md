---
id: ide-app-document-flow-ideapp-reabrir-no-canvas
kind: doc
target_file: src/ide_app/document_flow.rs
line_hint: 63
author: 
created: 
title: "IdeApp::reabrir_no_canvas"
tags: []
signature: "pub (crate) fn reabrir_no_canvas (& mut self , cx : & mut Cx)"
---

Delivers the entire document to the canvas runtime.

Reopening is the honest path here, and is the same as the example: the `touched`
of `Applied` is lost in the gesture loop, and a form has dozens of
nodes — `open` costs a single traversal and does not leave drawing behind
relative to the model.

#### At `rt.set_grid(if self.ver_grade {`

THE GRID, and the Options `Display grid` decides whether it appears.
Zero turns it off: `calcular_pontos_grade` returns an empty list below
2 (`crawrad-runtime-makepad/src/designer/grade.rs:7`), so
hiding the grid does not require a second drawing path — and the
SIZE the user chose remains preserved, which is what allows
re-checking the box to restore the grid they had.

#### At `rt.com_desenhos(crawrad_project::installed::declared_drawings());`

**The DECLARED drawings**, before opening the document.

The adapter intentionally does not depend on the catalog (ADR 0027), so
whichever entity HAS the catalog provides the data. Without this, a visual
third-party component would once again appear as a dashed rectangle — which is
what it was until 2026-09-18.
