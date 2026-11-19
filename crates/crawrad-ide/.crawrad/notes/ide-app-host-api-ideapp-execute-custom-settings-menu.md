---
id: ide-app-host-api-ideapp-execute-custom-settings-menu
kind: doc
target_file: src/ide_app/host_api.rs
line_hint: 23
author: 
created: 
title: "IdeApp::execute_custom_settings_menu"
tags: []
signature: "pub (in crate :: ide_app) fn execute_custom_settings_menu (& mut self , cx : & mut Cx , label : & str)"
---

Executes custom actions of the Activity Rail gear popup menu.

## Inside `execute_custom_settings_menu`: `if let Some(page) = options::page_of_menu_row(label) {`

The row's page of the Options, or the sentence a row with no wire
yet answers: both are tables outside the shell (#558).
