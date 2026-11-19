---
id: ide-app-build-ideapp-lancar-projeto
kind: doc
target_file: src/ide_app/build.rs
line_hint: 623
author: 
created: 
title: "IdeApp::lancar_projeto"
tags: []
signature: "pub (super) fn lancar_projeto (& mut self , cx : & mut Cx , artefatos : & [crate :: view_models :: diagnostics_model :: CompiledArtifact] ,)"
---

Launches the already compiled application, **without waiting for it**.

`cargo run` after a green `cargo build` recompiles nothing: it just
runs. And [`crawrad_os::process::detach`] instead of waiting is what separates
*running* from *frozen* — the IDE remains alive while the user's form
is open, which is how a classic RAD IDE behaves.

#### At `if !crawrad_project::workspace::has_binary(&raiz, &pacote) {`

THE BELT. The buckle is the catalog's `habilitada` — except in this shell
it receives a [`crawrad_actions::Projeto::default`], which is OPEN by
design: without a response from `cargo`, the IDE does not fabricate a verdict. So
here is the only real question, and it prevents the outcome that
Amendment 3 of ADR 0018 fixes: `error: a bin target must be
available for cargo run` in the face of whoever pressed `F5` — and in an
application LAUNCHED WITHOUT A CONSOLE that sentence appears nowhere.

## Inside `lancar_projeto`: `match crawrad_os::process::detach(`

The arguments come from the SAME [`crawrad_os::process::CargoCall`] that the
build uses, and not from a second hand-written list — that was how
issue #67 began.

## Inside `lancar_projeto`: `if self.build.mode != crate::ide_app::dialogos::build::BuildMode::R...`

Only Run mode (F9 / ▶ Run) executes the user's application.
Build, Check, and Clean modes only compile, check, or clean without running the app.

## Inside `lancar_projeto`: `let mut sem_depurador = String::new();`

UNDER THE DEBUGGER, unless requested without (issue #276). Without
the IDE's debugger installed, it runs as usual and says why.
