---
id: ide-app-link-ideapp-abrir-no-codigo
kind: doc
target_file: src/ide_app/link.rs
line_hint: 132
author: 
created: 
title: "IdeApp::abrir_no_codigo"
tags: []
signature: "pub (super) fn abrir_no_codigo (& mut self , cx : & mut Cx , handler : & str)"
---

Shows the user unit with the reader cursor at the requested handler.

# The one writing the `fn` is the GENERATOR, always

There is no `fn` snippet assembled here, and it is the same decision as the egui
shell (`Ide::navegar_para_handler`): the generator knows the mark inside the
`impl` and knows not to duplicate definitions (`crawrad_codegen::Adicao`). A
snippet shoved from here would be a second writer of the same unit, with its
own idea of where the `impl` starts.

Therefore the order is: **search on disk → not found → generate → search
again**. Generating without need would touch the `mtime` of everything and force
`cargo` to rebuild the world on the next `Run`.

# The declared limit

The code view of this shell is a READ-ONLY `CodeView`: line and
column go to the status bar, and there is no cursor to land. When
this shell gains an editor, this is where landing comes in —
[`crawrad_codegen::elo::ponto_de_digitacao`] already returns the byte.

#### At `self.abrir_no_editor(cx, unidade.clone());`

THE TAB SWITCHES EVEN WITHOUT OUTCOME: the user asked to see the code, and
it is the view that explains what is missing. It is the UNIT's tab — the same
one that `F12` opens, through the same door, so that the link does not create a
second tab over the file that `F12` had already opened.

## Inside `abrir_no_codigo`: `self.pouso_pendente = local.as_ref().map(|(p, _, _)| *p);`

AND THE CURSOR LANDS IN THE BODY, which is the entire gesture. It remains PENDING
for the next frame — see [`IdeApp::pouso_pendente`].

## Inside `abrir_no_codigo`: `if self`

Generation refused, and its phrase explains why. Overwriting it
with "handler not found" would swap the cause for the symptom.
