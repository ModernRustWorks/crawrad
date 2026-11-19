---
id: ide-app-dialogs-options-ideapp-mudar-grade
kind: doc
target_file: src/ide_app/dialogs/options.rs
line_hint: 908
author: 
created: 
title: "IdeApp::mudar_grade"
tags: []
signature: "pub (in super :: super) fn mudar_grade (& mut self , cx : & mut Cx , nova : f32)"
---

Sets the designer grid size within valid bounds.

The grid size, 2..=64 as in the classic RAD IDE: below 2 the grid points
come back empty (`crawrad-runtime-makepad/src/designer/grade.rs:7`).
