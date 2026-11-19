---
id: migrar-handlers-renomeados
kind: doc
target_file: src/lib.rs
line_hint: 568
author: 
created: 
title: "migrar_handlers_renomeados"
tags: []
signature: "fn migrar_handlers_renomeados (atual : & str , adicoes : & [Adicao] , vizinhas : & [String] , generated_before : & str ,) -> String"
---

Renames handler stubs in the user file when a component is renamed.

Renames handler stubs in the user file when a component has been renamed,
or when the form rebound an event from a handler the generated code called
(`generated_before`) to a new name: `event.rename`.
