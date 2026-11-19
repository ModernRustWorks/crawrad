---
id: ide-app-explorer-actions-ideapp-follow-project-framework
kind: doc
target_file: src/ide_app/explorer_actions.rs
line_hint: 32
author: 
created: 
title: "IdeApp::follow_project_framework"
tags: []
signature: "pub (super) fn follow_project_framework (& mut self)"
---

Syncs generator target with the open project's framework.

The generator target follows the open project's framework (#565),
read from its manifest when the project changes — not a preference.
