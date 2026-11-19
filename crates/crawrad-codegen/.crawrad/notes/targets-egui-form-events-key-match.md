---
id: targets-egui-form-events-key-match
kind: doc
target_file: src/targets/egui/form_events.rs
line_hint: 553
author: 
created: 
title: "key_match"
tags: []
signature: "fn key_match () -> String"
---

The `match` from egui's `Key` to ours, written out.

It is built from [`crate::events::key_variants`] and not typed here, and
that is the whole point: our vocabulary **is** egui's list, adopted whole
(the reason is in `crate::events`), so a second copy of 117 names would be
a second place to forget one. Parsing the one we already emit means the
conversion cannot name a key the contract does not have.
