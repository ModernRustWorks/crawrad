---
id: ide-app-packages-app-ideapp-escrever-cromo-dos-pacotes
kind: doc
target_file: src/ide_app/packages_app.rs
line_hint: 229
author: 
created: 
title: "IdeApp::escrever_cromo_dos_pacotes"
tags: []
signature: "pub (super) fn escrever_cromo_dos_pacotes (& mut self , cx : & mut Cx)"
---

Updates chrome elements, tab labels, and status bar for the package manager.

Writes the Package Manager chrome: the two tab labels, the target
package, and the panel status bar.

Like the rest of this shell, only when text CHANGES: rewriting a label
every frame requests a redraw every frame (`label.rs:247-249`), and
here that would mean a window that never sleeps while the panel
is in view.
