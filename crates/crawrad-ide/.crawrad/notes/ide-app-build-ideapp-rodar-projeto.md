---
id: ide-app-build-ideapp-rodar-projeto
kind: doc
target_file: src/ide_app/build.rs
line_hint: 444
author: 
created: 
title: "IdeApp::rodar_projeto"
tags: []
signature: "pub (super) fn rodar_projeto (& mut self , cx : & mut Cx)"
---

Generates, compiles and runs the project — the `Run` of a classic RAD IDE.

Generating comes first on purpose: running what was on disk while the
design on screen is already different is the kind of lie that costs an hour of
debugging.

Compiling goes from here to a thread ([`crawrad_os::process`]) and returns via the
clock ([`IdeApp::bombear_build`]). **`cargo run` never enters that
thread**: the call would only return when the user's application
CLOSED, and the bar would keep saying `building…` with the application in
front of whoever pressed the button — this was measured in the launcher header.

#### At `let Some((raiz, pacote)) = self.alvo_do_cargo() else {`

At the workspace ROOT and with `-p <member>`: a single `Cargo.lock` and `target/`
for the whole group, which is how Cargo treats a workspace
(ADR 0017, Amendment 1). What writes the arguments is
[`crawrad_os::process::CargoCall`], and nobody else.
