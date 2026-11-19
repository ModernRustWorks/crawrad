---
id: ide-app-extension-views-ideapp-extension-tool
kind: doc
target_file: src/ide_app/extension_views.rs
line_hint: 38
author: 
created: 
title: "IdeApp::extension_tool"
tags: []
signature: "pub (super) fn extension_tool (& mut self , cx : & mut Cx , name : & str , args : & serde_json :: Value ,) -> Result < String , String >"
---

Dispatches extension installation or uninstallation tool calls.

`project.install_extension {path}` / `project.uninstall_extension
{id}` (MCP): the panel's two doors; the status bar's sentence answers.
