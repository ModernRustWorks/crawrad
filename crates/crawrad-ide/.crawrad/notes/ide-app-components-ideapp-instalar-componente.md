---
id: ide-app-components-ideapp-instalar-componente
kind: doc
target_file: src/ide_app/components.rs
line_hint: 25
author: 
created: 
title: "IdeApp::instalar_componente"
tags: []
signature: "pub (super) fn instalar_componente (& mut self , cx : & mut Cx , folder : Option < PathBuf >)"
---

Installs a component package.

**Install a component package.**

The picker is the system native one (ADR 0017), the same as `New Project…`.
Canceling it **is not a refusal**: there is no message to show, and reopening the
dialog on top of someone who just canceled would be the IDE refusing to take
"no" for an answer.

With a `path` (an agent over MCP), the folder is installed straight away, no picker.
