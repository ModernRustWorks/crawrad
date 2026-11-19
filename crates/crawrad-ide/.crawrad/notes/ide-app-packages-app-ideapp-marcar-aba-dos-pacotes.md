---
id: ide-app-packages-app-ideapp-marcar-aba-dos-pacotes
kind: doc
target_file: src/ide_app/packages_app.rs
line_hint: 219
author: 
created: 
title: "IdeApp::marcar_aba_dos_pacotes"
tags: []
signature: "pub (super) fn marcar_aba_dos_pacotes (& mut self , cx : & mut Cx)"
---

Selects the tab button that state says is in front.

Only the ACTIVE one is selected: `RadioButton`'s `unselect` is private
(`radio_button.rs:1406`), and the others are already born unselected — what
unselects them later is `RadioButtonSet` itself upon seeing the click
(`:1420-1433`). It is the same gesture that `montar_cromo` performs on the Object
Inspector tabs.

Without this, the panel would start with NEITHER of the two lit, and a search
triggered from within the `Installed` tab would leave the highlight on the wrong tab.
