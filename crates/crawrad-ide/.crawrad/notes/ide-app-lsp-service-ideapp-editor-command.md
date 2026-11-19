---
id: ide-app-lsp-service-ideapp-editor-command
kind: doc
target_file: src/ide_app/lsp_service.rs
line_hint: 261
author: 
created: 
title: "IdeApp::editor_command"
tags: []
signature: "pub (super) fn editor_command (& mut self , cx : & mut Cx , command : Option < crawrad_actions :: EditorCommand > ,)"
---

Executes an editor command dispatched from menus, keys, or MCP.

What the editor is asked to do (`EditorCommand`): from the catalogue
(and so from MCP) and from the editor's own keys alike.

## Inside `editor_command`: `if cv.bookmarks_owner() != Some(file.as_path()) {`

The view starts from the session's, and the session takes
the result at once: no frame passes with the two disagreeing.
