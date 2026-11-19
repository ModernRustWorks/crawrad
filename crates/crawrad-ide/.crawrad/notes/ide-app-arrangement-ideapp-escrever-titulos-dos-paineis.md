---
id: ide-app-arrangement-ideapp-escrever-titulos-dos-paineis
kind: doc
target_file: src/ide_app/arrangement.rs
line_hint: 419
author: 
created: 
title: "IdeApp::escrever_titulos_dos_paineis"
tags: []
signature: "pub (super) fn escrever_titulos_dos_paineis (& mut self , cx : & mut Cx)"
---

The TITLE of each dockable panel, from the catalog.

# Why it came back

It used to be intentionally empty, and the reason was good: what named the
panel was the dock TAB, and writing it in the header as well placed the same
word twice in the same column — the owner flagged this in a screenshot.

The reason fell away along with the tab strip, collapsed at his request (the
rail is already navigation). Without the tab, the panels were left with NO NAME at all
on the screen, and he demanded: *"Inspector title, please"*.

It is the house rule that this illustrates: **what is removed, is later
forgotten**. The note justifying the emptiness survived its reasoning for
an entire cycle.

# From the catalog, not from the DSL

The name of a panel is `crawrad_actions::Panel::label`. Written in the DSL,
it would put the list of panels in two places, with the second being born
outdated on the day the first changed — the same reason as
`escrever_rotulos_das_abas` right next to it.

## Inside `escrever_titulos_dos_paineis`: `if matches!(p, Panel::Components | Panel::Chat) {`

TWO are left out, and both because they have their OWN names in the DSL:

- `Components` holds TWO stacked sections (`Toolbox` and
  `Structure`), and neither is called "Components" — writing
  the panel name there would overwrite the top section's name;
- `Chat` is named `Aefos AI`, which is a PRODUCT name and
  not the panel's.

`Design` has no header — the center is the canvas —, and
`set_text` on a path that does not exist does nothing.

## Inside `escrever_titulos_dos_paineis`: `self.dock()`

The path has ONE segment: each of these has only one header,
and searching for `title` finds its own without a path table
that would become obsolete the day a panel was renamed.
