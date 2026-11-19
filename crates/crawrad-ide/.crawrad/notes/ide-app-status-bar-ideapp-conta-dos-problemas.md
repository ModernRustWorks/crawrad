---
id: ide-app-status-bar-ideapp-conta-dos-problemas
kind: doc
target_file: src/ide_app/status_bar.rs
line_hint: 124
author: 
created: 
title: "IdeApp::conta_dos_problemas"
tags: []
signature: "fn conta_dos_problemas (& self) -> (String , String)"
---

Formats current error and warning counts for status bar display.

rust-analyzer's live count, the Explorer's own source (#412), so the two
never disagree; the last build's before it speaks; `—` before either.
