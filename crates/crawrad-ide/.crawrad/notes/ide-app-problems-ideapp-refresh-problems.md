---
id: ide-app-problems-ideapp-refresh-problems
kind: doc
target_file: src/ide_app/problems.rs
line_hint: 18
author: 
created: 
title: "IdeApp::refresh_problems"
tags: []
signature: "pub (super) fn refresh_problems (& mut self , cx : & mut Cx , failed : Option < (bool , Duration) >)"
---

Takes the last build's diagnostics; `failed` is `(run, elapsed)` when it failed.
