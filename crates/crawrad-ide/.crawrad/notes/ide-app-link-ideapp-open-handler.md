---
id: ide-app-link-ideapp-open-handler
kind: doc
target_file: src/ide_app/link.rs
line_hint: 29
author: 
created: 
title: "IdeApp::open_handler"
tags: []
signature: "pub (super) fn open_handler (& mut self , cx : & mut Cx , jump : & crawrad_actions :: HandlerJump)"
---

Lands in the handler an action asked for.

This form's own goes the way of a double click. The base's opens the base
form's unit, beside this one, and is only looked for: nothing is generated
there, since that file belongs to another form.
