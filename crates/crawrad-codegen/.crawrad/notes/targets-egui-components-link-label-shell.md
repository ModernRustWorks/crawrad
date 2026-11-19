---
id: targets-egui-components-link-label-shell
kind: doc
target_file: src/targets/egui/components/link_label.rs
line_hint: 7
author: 
created: 
title: "SHELL"
tags: []
signature: "const SHELL : & str = include_str ! (\"../../../../../crawrad-runtime/src/shell.rs\")"
---

The guarded opener embedded into the emitted form.

The egui app depends on nothing of ours: the guarded opener goes out with
the form, the very source `crawrad_runtime` compiles (#500).
