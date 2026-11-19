---
id: ide-app-document-flow-ideapp-colocar
kind: doc
target_file: src/ide_app/document_flow.rs
line_hint: 207
author: 
created: 
title: "IdeApp::colocar"
tags: []
signature: "pub (crate) fn colocar (& mut self , cx : & mut Cx , classe : & str , x : f32 , y : f32)"
---

Inserts the armed component into the container under the point.

Dropping the armed class from the palette — **inside whichever container is under
the point**.

Via the CATALOG, and not through a transaction written here: `component.add` is
the literal example of ADR 0005, and is what gives the RTA agent the same
path that the mouse has.

# `parent` was missing, and with it half of a classic RAD IDE was missing

Until 2026-09-17, this function sent only `class`, `x`, and `y`. The action treats
a missing `parent` as *"put on the form"*, so **dropping on top of
a `Panel`, a `GroupBox`, or a `ScrollBox` placed the component BESIDE
it**, as a sibling — and in a classic RAD IDE, dropping inside a container is
how a form is built.

Measured when attempting to prove the `ScrollBox` bar: the button dropped inside the
box was created as a child of the form, and the box remained empty.

What determines which container receives it is `Designer::pai_para_soltar`,
which already existed in full and was already called by the egui shell
(`main.rs:4394`): it knows the catalog's `area_para_filhos`, the innermost one
wins, breaks ties by Z-order, and **returns the position already relative
to the parent**. This shell does not recalculate any of that (ADR 0027).
