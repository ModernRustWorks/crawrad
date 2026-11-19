---
id: targets-egui-form-events-state-head
kind: doc
target_file: src/targets/egui/form_events.rs
line_hint: 584
author: 
created: 
title: "STATE_HEAD"
tags: []
signature: "const STATE_HEAD : & str = r#\"\n/// What the form's own events remember between frames.\n///\n/// `OnShow` fires once, `OnResize` only when the size really changed, and\n/// `OnMouseEnter`/`OnMouseLeave` are edges — all of them need to know what the\n/// previous frame looked like, and a frame has nowhere else to keep it.\n#[derive(Clone, Copy, Debug, Default)]\nstruct FormState {\n\"#"
---

The head of the FormState struct definition.

The head of the state struct. The FIELDS are chosen one by one by
[`form_events`], and only what some wired event actually reads comes out.
