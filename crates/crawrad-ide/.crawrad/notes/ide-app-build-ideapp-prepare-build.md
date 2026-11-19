---
id: ide-app-build-ideapp-prepare-build
kind: doc
target_file: src/ide_app/build.rs
line_hint: 348
author: 
created: 
title: "IdeApp::prepare_build"
tags: []
signature: "pub (super) fn prepare_build (& mut self , cx : & mut Cx) -> bool"
---

What `Run`, `Build` and `Check` do before `cargo`.

With a form open, it is the `Generate`, and the generator's refusal (its
prefix in the status) stops the caller: `cargo` must not compile code that
was never written. With no form (a console app, #779) there is nothing to
generate, so it only saves what is typed. `false`: stop, the status says why.
