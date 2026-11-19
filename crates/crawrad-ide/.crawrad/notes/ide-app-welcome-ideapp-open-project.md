---
id: ide-app-welcome-ideapp-open-project
kind: doc
target_file: src/ide_app/welcome.rs
line_hint: 35
author: 
created: 
title: "IdeApp::open_project"
tags: []
signature: "pub (super) fn open_project (& mut self , cx : & mut Cx , folder : Option < PathBuf >)"
---

Opens a project folder directly or prompts via the system folder picker.

`project.open_folder` (#555): the folder named, else the folder picker; a
form open with unsaved work is never dropped for it.
