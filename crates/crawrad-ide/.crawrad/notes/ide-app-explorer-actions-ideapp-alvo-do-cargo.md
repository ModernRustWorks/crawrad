---
id: ide-app-explorer-actions-ideapp-alvo-do-cargo
kind: doc
target_file: src/ide_app/explorer_actions.rs
line_hint: 59
author: 
created: 
title: "IdeApp::alvo_do_cargo"
tags: []
signature: "pub (crate) fn alvo_do_cargo (& self) -> Option < (std :: path :: PathBuf , String) >"
---

Determines where cargo runs and which package it compiles.

**Where `cargo` runs and what it compiles** — both together, because one
without the other does not assemble any call.

The folder is the group ROOT and the package is the MEMBER's: one
`Cargo.lock` and one `target/` for the whole group, which is how Cargo
treats a workspace (ADR 0017, Amendment 1). Without a derived member, the
root answers for it — a standalone package opened as a folder is the sole
member of its own group (Amendment 1 again).

One method rather than two calls in each place: `Run` constructs this
pair twice, once to compile and once to launch, and both MUST specify the
same package. Written separately, they diverge — which is literally
issue #67, one scale down.

**#779 — a project with no form.** With no form in the designer, the file
in the editor says which crate is built (a console app): the form in the
host first, then the editor tab, then the folder the IDE opened. Only a FORM
counts from the host: after `project.open` its active file is the project
FOLDER, which has no member, and `cargo build -p app` was the result.

#### At `let pacote = crawrad_project::generator::package_name_in(&membro);`

Through `generator`, which is the one that already knows how to
fall back to the fallback `"app"` when the manifest cannot be read —
and it is the SAME name that generated code uses in `use`. Two names
here would mean the `cargo build -p` of one package and the `use`
of another.
