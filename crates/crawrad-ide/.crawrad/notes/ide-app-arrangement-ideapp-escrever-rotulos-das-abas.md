---
id: ide-app-arrangement-ideapp-escrever-rotulos-das-abas
kind: doc
target_file: src/ide_app/arrangement.rs
line_hint: 445
author: 
created: 
title: "IdeApp::escrever_rotulos_das_abas"
tags: []
signature: "pub (super) fn escrever_rotulos_das_abas (& mut self , cx : & mut Cx)"
---

Writes the label of every open tab, from the catalog.

The DSL intentionally leaves `name` BLANK: a panel's name is
`crawrad_actions::Panel::label`, and writing it in the DSL as well would mean the list of
panels in two places — with the second born outdated the day
the first changed.
