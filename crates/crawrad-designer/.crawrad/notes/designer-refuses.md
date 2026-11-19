---
id: designer-refuses
kind: doc
target_file: src/lib.rs
line_hint: 310
author: 
created: 
title: "Designer::refuses"
tags: []
signature: "pub fn refuses (& self , doc : & Document , tx : & Transaction) -> bool"
---

Returns whether the lock refuses this transaction.

Does the lock refuse this transaction? **This is the gate.**

It lives in whoever APPLIES, not in whoever produces, because producers of
`SetRect` exist in thirteen places and one gate per producer is the rule written
thirteen times — with the fourteenth born without it. The first version
of this command erred exactly like that: guarded the five drag `SetRect`
(`pointer_move`, `cancel`, `nudge`) and let pass the four from
`arrangement` — `Align ▸ Left`, `Size ▸ Same width`, `In form ▸ Center`
moved the entire form with the strip saying "controls locked". A
FALSE state on the screen is worse than no state.

Exhaustive `match` on [`Command`]: a new command does not compile without
someone deciding which side it falls on, instead of inheriting "allowed" by
omission — which is how the previous loophole would have returned.

**The whole transaction is refused, not filtered.** Half `Align ▸ Left`
— the unlocked move, the locked stay — is damage that the user
did not ask for and does not see coming.

Three things it deliberately DOES NOT reach:

1. **undo and redo**, which do not pass through `apply` (the shell calls
   `Doc::undo`/`Doc::redo`). A `Lock Controls` that locks Ctrl+Z is
   worse than none;
2. **the layout pass**, for the reason written in `locked_for`;
3. **the Object Inspector**, which never emits `SetRect` — it writes
   `SetProp`, and it is that line of the `match` that keeps typed Left, Top, Width and
   Height editable. It is Lock *Controls*, not Lock Properties.

# Examples

```rust
use crawrad_designer::Designer;
use crawrad_doc::Transaction;
use crawrad_ir::Document;

let d = Designer::default();
let doc = Document::new("form", "Form");
let tx = Transaction::user("test");
assert!(!d.refuses(&doc, &tx));
```
