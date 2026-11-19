---
id: events-base-arm
kind: doc
target_file: src/events.rs
line_hint: 830
author:
created:
title: "base_arm"
tags: []
signature: "pub (crate) fn base_arm (doc : & Document , form_type : & str , base_type : & str , base_events : & [WiredEvent] ,) -> Result < String , Erro >"
---

The `dispatch` arm of a form's `Base` variant: overrides, then the base.

An event this form overrides runs its handler; any other goes down to the
base's own `dispatch`, which applies the base's overrides in turn.
