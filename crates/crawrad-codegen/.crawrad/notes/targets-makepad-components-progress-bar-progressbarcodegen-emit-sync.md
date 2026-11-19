---
id: targets-makepad-components-progress-bar-progressbarcodegen-emit-sync
kind: doc
target_file: src/targets/makepad/components/progress_bar.rs
line_hint: 55
author: 
created: 
title: "ProgressBarCodegen::emit_sync"
tags: []
signature: "fn emit_sync (& self , doc : & Document , node : & Node , field : & str) -> String"
---

Emits synchronization code for progress bar position and range.

#436: the inner `bar` sized to the position's fraction of the range
(`crawrad_runtime_makepad::progress`); the control's design size is the
whole. `step` and `smooth` are not drawn yet — they stay read here.
