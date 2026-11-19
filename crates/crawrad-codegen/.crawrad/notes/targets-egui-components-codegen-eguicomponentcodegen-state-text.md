---
id: targets-egui-components-codegen-eguicomponentcodegen-state-text
kind: doc
target_file: src/targets/egui/components/codegen.rs
line_hint: 15
author: 
created: 
title: "EguiComponentCodegen::state_text"
tags: []
signature: "fn state_text (& self) -> Option < StateText >"
---

The state type this class pastes into `.designer.rs`.

The state type this class pastes into the `.designer.rs` (#124): the
text around the struct the catalogue derives. `None` for a component
that brings its own type from a crate.
