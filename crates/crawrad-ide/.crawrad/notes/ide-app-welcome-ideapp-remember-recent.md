---
id: ide-app-welcome-ideapp-remember-recent
kind: doc
target_file: src/ide_app/welcome.rs
line_hint: 57
author: 
created: 
title: "IdeApp::remember_recent"
tags: []
signature: "pub (super) fn remember_recent (folder : & Path)"
---

Prepends the opened project directory to the recent projects list on disk.

The folder goes to the top of the recent list on disk — never from
`cargo test`, whose forms live in temp folders the user never opened.
