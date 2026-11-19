---
id: ide-app-rename-ideapp-apply-file-changes
kind: doc
target_file: src/ide_app/rename.rs
line_hint: 193
author: 
created: 
title: "IdeApp::apply_file_changes"
tags: []
signature: "pub (super) fn apply_file_changes (& mut self , cx : & mut Cx , changes : & [FileChange])"
---

Applies server file changes in memory with single undo step tracking.

Edits of the server applied in memory (a rename, a code action): the
file in front as ONE undo step; the others become unsaved edits whose
undo is replayed when they are shown, and the server is told.
