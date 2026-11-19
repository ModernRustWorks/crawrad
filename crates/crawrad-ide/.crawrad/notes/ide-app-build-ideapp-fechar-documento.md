---
id: ide-app-build-ideapp-fechar-documento
kind: doc
target_file: src/ide_app/build.rs
line_hint: 300
author: 
created: 
title: "IdeApp::fechar_documento"
tags: []
signature: "pub (super) fn fechar_documento (& mut self , cx : & mut Cx)"
---

Drops the open form — **the folder remains open**.

The state in which it leaves the IDE already exists and is normal: it is the same one where
the first run without a form stops, and the bar already knows how to display it
([`documento::file_label`]).

#### At `if self`

THE GUARD BEFORE DROPPING, and it comes after committing the field being
edited: without this, closing with a half-typed `Name` in the Inspector would say
`saved` and carry the rename along. It is the same order as the other gates
in the codebase.
