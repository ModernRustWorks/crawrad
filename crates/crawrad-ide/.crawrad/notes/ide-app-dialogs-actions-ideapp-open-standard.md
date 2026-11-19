---
id: ide-app-dialogs-actions-ideapp-open-standard
kind: doc
target_file: src/ide_app/dialogs/actions.rs
line_hint: 36
author: 
created: 
title: "IdeApp::open_standard"
tags: []
signature: "pub (in super :: super) fn open_standard (& mut self , cx : & mut Cx)"
---

Opens the standard classes window, from inside the ActionList Editor.

**Which categories start closed is decided HERE, once**, and not on
every draw: the set keeps what is closed, and an empty set
meaning *"all open"* would make the window open with thirty classes
in view. The rule for which ones they are belongs to the model
([`action_list::group_starts_open`]), which is the same as egui.
