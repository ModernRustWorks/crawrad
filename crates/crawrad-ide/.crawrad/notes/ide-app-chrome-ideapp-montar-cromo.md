---
id: ide-app-chrome-ideapp-montar-cromo
kind: doc
target_file: src/ide_app/chrome.rs
line_hint: 1
author: 
created: 
title: "IdeApp::montar_cromo"
tags: []
signature: "pub (super) fn montar_cromo (& mut self , cx : & mut Cx)"
---

Writes the parts of the window chrome that never change with the document.

## Inside `montar_cromo`: `self.load_arrangement(cx);`

The saved arrangement, if there is one that can be trusted.

It comes LAST, and the chrome inside the panels comes AFTER it,
in `repovoar_paineis`: `load_state` recreates the widget for each dock
item, and whatever was written before is lost without any error.

## Inside `montar_cromo`: `for c in toolbar::Command::ALL {`

THE CHROME GLYPHS. None of them is written in the DSL: the codepoint
goes via `\u{}` in Rust, where it is readable (see `icon.rs`).
THE COMMAND STRIP: the glyph for each one comes from its role, and the role
comes from `toolbar::Command` — which is also what knows which action the button
triggers. A single place for the entire strip.

## Inside `montar_cromo`: `for m in menu::Menu::ALL {`

THE SIX MENU TITLES, from `menu::Menu::title()` — which is also the
`Action::grupo` that each one draws. A title written in the DSL would mean
having the menu list in two places.

## Inside `montar_cromo`: `self.write_panel_chrome(cx);`

If there was no saved arrangement, `load_arrangement` does not call
`repovoar_paineis` — so the chrome is written here. Calling it twice
is cheap, and writing it zero times leaves the tab strip blank.

## Inside `montar_cromo`: `self.escrever_icones_do_rodape(cx);`

THE FOOTER GLYPHS, once: they do not change with the document, and
paying for them every frame would require a redraw every frame.
