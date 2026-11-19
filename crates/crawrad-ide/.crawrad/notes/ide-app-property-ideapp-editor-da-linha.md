---
id: ide-app-property-ideapp-editor-da-linha
kind: doc
target_file: src/ide_app/property.rs
line_hint: 75
author: 
created: 
title: "IdeApp::editor_da_linha"
tags: []
signature: "pub (super) fn editor_da_linha (& self , linha : usize) -> Option < inspector :: Editor >"
---

The editor that drew a line, reassembled from the catalogue.

The list is reassembled and not cached for the same reason that the menu is:
`Action::habilitada` and the value of each property depend on the
document, which changes underneath whoever has a field focused.
