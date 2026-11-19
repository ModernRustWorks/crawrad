---
id: ide-app-dialogs-options-ideapp-watch-agent-install
kind: doc
target_file: src/ide_app/dialogs/options.rs
line_hint: 938
author: 
created: 
title: "IdeApp::watch_agent_install"
tags: []
signature: "pub (crate) fn watch_agent_install (& mut self , name : & str , rx : AgentInstall)"
---

Monitors a background agent installation task.

An agent install started in the background (#424): the catalog says so
now, and [`Self::follow_agent_install`] shows how it ended.
