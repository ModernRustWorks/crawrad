---
id: ide-app-comp-rename-start
kind: doc
target_file: src/ide_app/component_rename.rs
line_hint: 21
author: 
created: 
title: "IdeApp::start_component_rename"
tags: []
signature: "pub(super) fn start_component_rename( &mut self, cx: &mut Cx, node_id: String, new_name: String, )"
---

Starts a Name change from the Inspector: asks rust-analyzer for the references.
