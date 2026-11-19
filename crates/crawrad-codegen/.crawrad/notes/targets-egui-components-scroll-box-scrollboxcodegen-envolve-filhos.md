---
id: targets-egui-components-scroll-box-scrollboxcodegen-envolve-filhos
kind: doc
target_file: src/targets/egui/components/scroll_box.rs
line_hint: 26
author: 
created: 
title: "ScrollBoxCodegen::envolve_filhos"
tags: []
signature: "fn envolve_filhos (& self) -> bool"
---

**It wraps**, and it is the only one in the target that wraps.

Without this it was an `egui::ScrollArea` with an EMPTY body, with the children
drawn as SIBLINGS in the form's `show()`. A container that does not
contain does not overflow, does not scroll, and does not show a bar — and the child that
exceeded the box appeared over its neighbor instead of being clipped by
it (issue #157).
