---
id: ide-app-recover-ideapp-follow-recovery
kind: doc
target_file: src/ide_app/recover.rs
line_hint: 10
author: 
created: 
title: "IdeApp::follow_recovery"
tags: []
signature: "pub (super) fn follow_recovery (& mut self , cx : & mut Cx)"
---

Tracks open project changes to initiate recovery monitoring and prompts.

Follows the open project, each frame: arriving at one starts the clock
and offers what a crash left there.
