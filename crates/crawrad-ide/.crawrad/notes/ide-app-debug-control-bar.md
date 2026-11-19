---
id: ide-app-debug-control-bar
kind: doc
target_file: src/ide_app/debug.rs
line_hint: 107
author: 
created: 
title: "CONTROL_BAR"
tags: []
signature: "const CONTROL_BAR : & [(& [LiveId] , icon :: Chrome , & str)] = & [(id ! (btn_dbg_continue) , icon :: Chrome :: DebugContinue , \"project.run\" ,) , (id ! (btn_dbg_pause) , icon :: Chrome :: DebugPause , \"debug.pause\") , (id ! (btn_dbg_step_over) , icon :: Chrome :: StepOver , \"debug.step_over\" ,) , (id ! (btn_dbg_step_into) , icon :: Chrome :: StepInto , \"debug.step_into\" ,) , (id ! (btn_dbg_step_out) , icon :: Chrome :: StepOut , \"debug.step_out\" ,) , (id ! (btn_dbg_stop) , icon :: Chrome :: DebugStop , \"debug.stop\") ,]"
---

Control bar button definitions for Run and Debug.

The control bar of Run and Debug: each button, the role of its glyph, and
the catalogue action it runs — the same action the scheme's key runs.
