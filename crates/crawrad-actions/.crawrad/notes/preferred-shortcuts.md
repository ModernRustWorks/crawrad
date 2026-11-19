---
id: preferred-shortcuts
kind: doc
target_file: src/lib.rs
line_hint: 5973
author: 
created: 
title: "PREFERRED_SHORTCUTS"
tags: []
signature: "pub const PREFERRED_SHORTCUTS : & [(& str , & [& str])] = & [(\"view.ui_zoom_in\" , & [\"Ctrl+=\" , \"Ctrl+Shift+=\" , \"Ctrl+NumpadAdd\"] ,) , (\"view.ui_zoom_out\" , & [\"Ctrl+-\" , \"Ctrl+NumpadSubtract\"]) , (\"view.ui_zoom_reset\" , & [\"Ctrl+0\" , \"Ctrl+Numpad0\"]) ,]"
---

The preferred shortcut chords mapped to action IDs.

**The chord that everyone expects, for the host that can deliver it.**

# Why a SECOND chord exists

Canvas zoom has a pair of keys everyone tries first:
`Ctrl +`, `Ctrl -` and `Ctrl 0`. The owner demanded exactly this on 17/09/2026,
looking at the IDE: *"zoom with Ctrl+ and Ctrl- of the IDE does not work yet"*.

Their [`Action::shortcut`] **cannot** be that pair, and the reason is
measured in `nenhum_atalho_do_catalogo_cai_no_zoom_da_ide_do_egui`: in the
**egui** host these four keys have an owner. `egui` consumes them at the
end of each pass (`Options::zoom_with_keyboard`) and moves the `zoom_factor`
of the entire interface — which is a real option, with a control in the Settings
window whose own tooltip says *"Same as Ctrl + and Ctrl -"*. Because that shell's
keyboard bridge **reads** events without consuming them, such a chord
in the catalogue would trigger BOTH things: canvas zoom and chrome zoom.

And choosing a neighbor does not help: egui's `matches_logically` ignores
EXTRA modifiers, so `Ctrl+Shift+=` falls into its handler as well.

# The Makepad shell has no such owner

There is no interface `zoom_factor` there and no one consumes those keys, so
the expected chord is free — and denying it to that shell because of a
limitation of the other would make the whole product pay the price of one host.

That is why the action carries TWO chords: [`Action::shortcut`], which every shell
delivers, and this one, which only those without an owner for the key deliver. **Both
live in the catalogue**, which is what [ADR 0011] demands — what it forbids is the
shell inventing keys, not the catalogue declaring two.

# Why each action has a LIST, and not a chord

Because *"Ctrl plus"* is not one key — it is three, and the owner encountered this on
screen: with `Ctrl+=` bound alone, *"makepad does not even move"*. The `+` on the main
keyboard IS `Shift` + `=`, so whoever presses what reads as `Ctrl +` sends
`Ctrl+Shift+=`; and whoever uses the numeric keypad sends an entirely
different key, which in Makepad is `NumpadAdd` (`event/keyboard.rs:247`).

VS Code binds all three, and browsers do too. Binding only one is the chord that
works for whoever wrote the code and for no one else.

# Examples

```rust
use crawrad_actions::PREFERRED_SHORTCUTS;

assert!(!PREFERRED_SHORTCUTS.is_empty());
```
