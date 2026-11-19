---
id: targets-makepad-form-events-key-match
kind: doc
target_file: src/targets/makepad/form_events.rs
line_hint: 468
author: 
created: 
title: "key_match"
tags: []
signature: "pub (super) fn key_match () -> String"
---

**Makepad's `KeyCode` onto our `Key`**, and here it is a TRANSLATION.

The egui twin is a rename — our vocabulary is egui's list adopted whole —
so it is generated from `key_variants` and cannot drift. This one cannot
be: Makepad names the same keys differently (`KeyQ`, `Key0`, `ReturnKey`)
and has keys our contract does not name at all.

Those are the interesting lines, and each one says why:

- `Capslock`, `Numlock`, `PrintScreen`, `ScrollLock` and `Pause` have no
  name in the contract, because egui does not report them either. They
  come through as `Key::Unknown` rather than as some neighbour;
- `NumpadMultiply` is the same case: there is no asterisk in the list;
- `Control`, `Alt`, `Shift` and `Logo` are reported by Makepad WITHOUT a
  side, while the contract has a left and a right. They map to the left
  one — the common one — and that is a loss, written down here rather than
  discovered by someone testing the right Shift;
- the numeric keypad reports the same `Key` as the number row. egui does
  not separate them, so neither does the contract.

There is no `_` arm, for the same reason as the twin: `KeyCode` is not
`#[non_exhaustive]`, so a key added to Makepad stops this compiling and
says which one.
