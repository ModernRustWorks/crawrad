---
id: targets-makepad-form-events-state-head
kind: doc
target_file: src/targets/makepad/form_events.rs
line_hint: 605
author: 
created: 
title: "STATE_HEAD"
tags: []
signature: "const STATE_HEAD : & str = r#\"\n/// What the form's own events remember between events.\n///\n/// `OnShow` fires once, `OnMouseEnter` is an edge, and a CLICK is a press and\n/// a release that have to be compared to each other — none of them can be\n/// answered by the event in hand alone.\n#[derive(Clone, Copy, Debug, Default)]\npub struct FormState {\n\"#"
---

The head definition of the generated form state struct.

The head of the state struct. The FIELDS are chosen one by one by
[`form_events`], and only what some wired event actually reads comes out.
