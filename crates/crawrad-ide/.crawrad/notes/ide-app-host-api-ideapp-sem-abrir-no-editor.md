---
id: ide-app-host-api-ideapp-sem-abrir-no-editor
kind: doc
target_file: src/ide_app/host_api.rs
line_hint: 51
author: 
created: 
title: "IdeApp::SEM_ABRIR_NO_EDITOR"
tags: []
signature: "pub (super) const SEM_ABRIR_NO_EDITOR : & 'static str = \"opening files in an editor is not wired in this shell yet — use the egui shell\""
---

THE THREE PHRASES FOR WHAT IS NOT YET WIRED IN THIS SHELL.

Constants and not literals in the match arm, for two measured reasons. The first
is that **the phrase is the only feedback the user receives** from a command
that does nothing, and a response that lives inside a two-hundred-line
`match` is a response that no one reviews. The second is that they
are the needle of the text gates (`test/ide_app/teclado.rs`,
`test/ide_app/fios.rs`): with the literal in the arm, changing a comma in
the phrase fails a test for a reason that is not its own.

And there are THREE, one per variant, and not a single shared one: the shared
phrase was exactly the `Some(_)` that swallowed `Run`
(issue #147).
