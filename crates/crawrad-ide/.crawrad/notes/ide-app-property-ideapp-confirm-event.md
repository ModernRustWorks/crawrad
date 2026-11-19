---
id: ide-app-property-ideapp-confirm-event
kind: doc
target_file: src/ide_app/property.rs
line_hint: 115
author: 
created: 
title: "IdeApp::confirm_event"
tags: []
signature: "pub (super) fn confirm_event (& mut self , cx : & mut Cx)"
---

Confirms a pending event handler edit in the Events tab.

Confirms the pending edit in the Events tab, on `Enter` or leaving the
field: [`crawrad_actions::handler_edit`] says whether it renames or binds.
