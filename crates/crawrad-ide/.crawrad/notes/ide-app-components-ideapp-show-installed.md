---
id: ide-app-components-ideapp-show-installed
kind: doc
target_file: src/ide_app/components.rs
line_hint: 47
author: 
created: 
title: "IdeApp::show_installed"
tags: []
signature: "pub (super) fn show_installed (& mut self , cx : & mut Cx)"
---

Shows what the catalog loaded and what it rejected.

**What the catalog loaded — and what it REJECTED.**

The second half is what matters: a package that did not make it in does not appear
anywhere in the IDE, and without this line the only clue would be the palette
missing the component that the user just installed.
