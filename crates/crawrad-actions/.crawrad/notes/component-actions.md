---
id: component-actions
kind: doc
target_file: src/lib.rs
line_hint: 4550
author: 
created: 
title: "component_actions"
tags: []
signature: "fn component_actions () -> Vec < Action >"
---

**The four mutations**: create, change property, rename, wire handler.

They entered together because together they are what separates a catalogue that
*describes* an IDE from one that *directs* it. Measured in commit `17d6d37`: the
catalogue had 33 actions and **none** began with `component.` — an agent
aligned, ordered, copied, deleted, generated, and ran, and could not put
a button on the form. It is the classic sin of OTA that §12.3 of the study
names, and as long as it existed the Phase 4 chat panel would be born
decorative: conversation about a form that cannot be touched.

The four divide three decisions, and it is worth writing them down once:

1. **the node comes by argument, and not from the selection.** The selection is window
   state; on the other side of the protocol there is no window at all. Whoever calls specifies
   which node they are touching, and that is why all four require an argument — which
   also takes them off the menus, which have nowhere to type one
   ([`Action::requires_argument`]).
2. **none repeats what `crawrad-doc` already validates.** `Command::Rename`
   refuses an empty name and a name with neither letter nor digit, and that gate is not
   written again here: a guard outside the single write gate is not a
   guard, it is a second rule to keep up to date. What these do is what
   `crawrad-doc` **cannot** do — give up early when the ARGUMENT
   was not provided, so the caller receives a phrase instead of a transaction that
   makes no sense.
3. **the label is what the status bar and Ctrl+Z show**, and therefore
   it is derived from the document (`btnOk.text`, `Rename btnOk to btnSave`) and
   not invented — it is the same text that the shell wrote by hand before.
