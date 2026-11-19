---
id: targets-makepad-components-codegen-makepadcomponentcodegen-app-state
kind: doc
target_file: src/targets/makepad/components/codegen.rs
line_hint: 54
author: 
created: 
title: "MakepadComponentCodegen::app_state"
tags: []
signature: "fn app_state (& self , _node : & Node , _field : & str) -> Result < Option < AppState > , Erro >"
---

State that the component keeps in the generated App struct.

What the component keeps in the generated `App` itself and does in its
`handle_event` — a platform object that is not form state (#437: the
`Timer`'s interval). Most components keep nothing.
