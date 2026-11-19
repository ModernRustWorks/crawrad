---
id: ide-app-recover-ideapp-answer-recovery
kind: doc
target_file: src/ide_app/recover.rs
line_hint: 41
author: 
created: 
title: "IdeApp::answer_recovery"
tags: []
signature: "pub (super) fn answer_recovery (& mut self , cx : & mut Cx , files : Vec < PathBuf > , discard : bool)"
---

Restores or discards autorecovery backups for the specified files.

`file.recover_unsaved` and `file.discard_recovery`: a recovered copy
opens as the file's unsaved edit, so the disk stays as it is.
