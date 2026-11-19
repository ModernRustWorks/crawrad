---
id: events-overrides-base
kind: doc
target_file: src/events.rs
line_hint: 822
author:
created:
title: "overrides_base"
tags: []
signature: "pub (crate) fn overrides_base (doc : & Document , base_events : & [WiredEvent]) -> bool"
---

Whether the derived form overrides any of the base's wired events.

Only then does its designer name the base's `Fired` type, in the dispatch's
match arms; importing it otherwise is an unused import in the user's project.
