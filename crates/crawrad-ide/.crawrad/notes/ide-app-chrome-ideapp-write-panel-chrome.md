---
id: ide-app-chrome-ideapp-write-panel-chrome
kind: doc
target_file: src/ide_app/chrome.rs
line_hint: 74
author: 
created: 
title: "IdeApp::write_panel_chrome"
tags: []
signature: "pub (super) fn write_panel_chrome (& mut self , cx : & mut Cx)"
---

The chrome that lives INSIDE a dockable panel.

Separated from [`IdeApp::montar_cromo`] because it has a different owner in
time: the window chrome — menus, command strip, gear — is born once and stays;
this one dies and is reborn on every `load_state` of the dock.
See [`IdeApp::repovoar_paineis`], which is what calls it afterwards.

#### At `self.alvo_na_tela.clear();`

THE HEADER TARGET — `btn_ok · Button` — disappears due to the CACHE, and not
due to the dock.

What writes it is `sincronizar`, and only when it CHANGES
(`alvo_na_tela`). When the widget is recreated, the text disappears from the screen
and the cache keeps saying that it is there: the label remains blank until the
user selects another component. Measured on 09/16/2026, upon
hiding and reopening the Inspector from the rail.

Forgetting what was on screen is the right fix: `sincronizar`
right below rewrites it, and what decides the text remains a single source.

#### At `self.escrever_x_dos_paineis(cx);`

THE X OF EACH DOCKABLE PANEL.

It is written by the DOCK, panel by panel, and not through a widget path
in the window: `self.ui.widget(...)` caches the `WidgetRef`
(`view.rs`, `WidgetCache`) and `Dock` recreates the item's widget on each
`load_state` -- it was this cache that erased the form from the screen, see
[`IdeApp::repovoar_paineis`].

#### At `self.tab_button(self.aba)`

The ACTIVE tab. Only it is checked: `unselect` on `RadioButton` is
private (`radio_button.rs:1406`), and the others start unchecked
— what unchecks them later is `RadioButtonSet` itself upon seeing the
click (`:1420-1433`).

## Inside `write_panel_chrome`: `self.write_center_tabs(cx);`

THE TWO CENTER TABS — `Design` and `Code`. Without this call the strip
starts BLANK: `show_center`, which writes them, only runs
when the view switches, and at startup nothing switches.

## Inside `write_panel_chrome`: `self.escrever_cromo_dos_pacotes(cx);`

And the Package Manager chrome, for the same reason: its two tab
labels come from `Tab::ALL`, which is the list that BOTH shells read.

## Inside `write_panel_chrome`: `for aba in InspectorTab::ALL {`

THE TWO OBJECT INSPECTOR TABS, from `InspectorTab::ALL`. Written
in the DSL, they would be the pair of tabs in two places.
