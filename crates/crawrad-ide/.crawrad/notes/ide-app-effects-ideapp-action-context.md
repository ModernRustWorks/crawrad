---
id: ide-app-effects-ideapp-action-context
kind: doc
target_file: src/ide_app/effects.rs
line_hint: 1
author: 
created: 
title: "IdeApp::action_context"
tags: []
signature: "pub (super) fn action_context (& self , args : crawrad_actions :: Args) -> crawrad_actions :: Context < '_ >"
---

What the action catalogue is told about the IDE right now.

One place builds it, so running an action and asking whether it may run
(`refusal`) read the same facts.

## Inside `action_context`: `clipboard: &self.transferencia,`

The LIVE clipboard of the shell, not a newly created one:
see the `transferencia` field.
