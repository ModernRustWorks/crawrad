---
id: ide-app-debug-ideapp-start-debugging
kind: doc
target_file: src/ide_app/debug.rs
line_hint: 41
author: 
created: 
title: "IdeApp::start_debugging"
tags: []
signature: "pub (super) fn start_debugging (& mut self , cx : & mut Cx , program : PathBuf , cwd : & Path ,) -> Result < () , String >"
---

Starts program execution under the IDE debugger.

Starts `program` under the IDE's own debugger with the session's
breakpoints. `Err` says why it could not, and the caller runs it
without the debugger instead.
